//! Colour themes: tokens, WCAG contrast, the default-theme rule, and the CSS
//! custom properties generated from the tokens.

/// The four site themes. `Skh` is the default dark theme, `Paper` the light one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Theme {
    Skh,
    TokyoNight,
    Gruvbox,
    Paper,
}

/// Colour tokens of one theme (`#rrggbb`). `mut_` is the muted text colour.
#[derive(Clone, Copy, Debug)]
pub struct Tokens {
    pub bg: &'static str,
    pub bar: &'static str,
    pub fg: &'static str,
    pub mut_: &'static str,
    pub bd: &'static str,
    pub focus: &'static str,
    pub green: &'static str,
    pub blue: &'static str,
    pub yellow: &'static str,
    pub magenta: &'static str,
    pub sel: &'static str,
    pub prose: &'static str,
}

impl Theme {
    pub const ALL: [Theme; 4] = [Theme::Skh, Theme::TokyoNight, Theme::Gruvbox, Theme::Paper];

    pub fn name(self) -> &'static str {
        match self {
            Theme::Skh => "skh",
            Theme::TokyoNight => "tokyo-night",
            Theme::Gruvbox => "gruvbox",
            Theme::Paper => "paper",
        }
    }

    pub fn from_name(name: &str) -> Option<Theme> {
        Theme::ALL.into_iter().find(|t| t.name() == name)
    }

    /// The next theme in `ALL` order, wrapping around.
    pub fn next(self) -> Theme {
        let i = Theme::ALL.iter().position(|t| *t == self).unwrap_or(0);
        Theme::ALL[(i + 1) % Theme::ALL.len()]
    }

    pub fn is_light(self) -> bool {
        self == Theme::Paper
    }

    /// Tokens from the spec (§5), adjusted after the design review for contrast.
    pub fn tokens(self) -> Tokens {
        match self {
            Theme::Skh => Tokens {
                bg: "#14171e", bar: "#0f1217", fg: "#d6dbe4", mut_: "#8b93a5", bd: "#5d6473",
                focus: "#8fd19e", green: "#8fd19e", blue: "#86adff", yellow: "#e3c27e",
                magenta: "#d7a6ff", sel: "#252b37", prose: "#cfd5df",
            },
            Theme::TokyoNight => Tokens {
                bg: "#1a1b26", bar: "#16161e", fg: "#c0caf5", mut_: "#8f96bd", bd: "#5d658b",
                focus: "#7aa2f7", green: "#9ece6a", blue: "#7aa2f7", yellow: "#e0af68",
                magenta: "#bb9af7", sel: "#24304f", prose: "#b4bde6",
            },
            Theme::Gruvbox => Tokens {
                bg: "#282828", bar: "#1d2021", fg: "#ebdbb2", mut_: "#b0a28f", bd: "#7a7064",
                focus: "#b8bb26", green: "#b8bb26", blue: "#83a598", yellow: "#fabd2f",
                magenta: "#d3869b", sel: "#353230", prose: "#e0d3b0",
            },
            Theme::Paper => Tokens {
                bg: "#f4f2ee", bar: "#e6e2da", fg: "#22252c", mut_: "#5a6070", bd: "#908b84",
                focus: "#276b3b", green: "#276b3b", blue: "#2852b8", yellow: "#8a5a00",
                magenta: "#8a3ea8", sel: "#ebe7df", prose: "#2e323a",
            },
        }
    }
}

/// Theme to use: a saved name if it is valid, otherwise follow the OS setting.
pub fn resolve(saved: Option<&str>, prefers_light: bool) -> Theme {
    saved.and_then(Theme::from_name).unwrap_or(if prefers_light { Theme::Paper } else { Theme::Skh })
}

fn luminance(hex: &str) -> f64 {
    let h = hex.trim_start_matches('#');
    let channel = |i: usize| {
        let c = u8::from_str_radix(&h[i..i + 2], 16).unwrap_or(0) as f64 / 255.0;
        if c <= 0.03928 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
    };
    0.2126 * channel(0) + 0.7152 * channel(2) + 0.0722 * channel(4)
}

/// WCAG 2.1 contrast ratio between two `#rrggbb` colours (1.0 to 21.0).
pub fn contrast(a: &str, b: &str) -> f64 {
    let (la, lb) = (luminance(a), luminance(b));
    let (hi, lo) = if la > lb { (la, lb) } else { (lb, la) };
    (hi + 0.05) / (lo + 0.05)
}

/// CSS custom properties for every theme, keyed by `html[data-theme]`.
pub fn css() -> String {
    Theme::ALL
        .iter()
        .map(|t| {
            let k = t.tokens();
            format!(
                "html[data-theme=\"{}\"]{{--bg:{};--bar:{};--fg:{};--mut:{};--bd:{};--focus:{};--green:{};--blue:{};--yellow:{};--magenta:{};--sel:{};--prose:{};color-scheme:{}}}\n",
                t.name(), k.bg, k.bar, k.fg, k.mut_, k.bd, k.focus, k.green, k.blue, k.yellow,
                k.magenta, k.sel, k.prose, if t.is_light() { "light" } else { "dark" }
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn names_round_trip() { for t in Theme::ALL { assert_eq!(Theme::from_name(t.name()), Some(t)); } }
    #[test] fn next_cycles_all_four() { assert_eq!(Theme::Paper.next(), Theme::Skh); assert_eq!(Theme::Skh.next(), Theme::TokyoNight); }
    #[test] fn resolve_rules() {
        assert_eq!(resolve(Some("gruvbox"), true), Theme::Gruvbox);
        assert_eq!(resolve(None, true), Theme::Paper);
        assert_eq!(resolve(None, false), Theme::Skh);
        assert_eq!(resolve(Some("dark"), false), Theme::Skh); // stale value from the old site
        assert_eq!(resolve(Some(""), true), Theme::Paper);
    }
    #[test] fn every_text_pair_meets_aa() {
        for t in Theme::ALL { let k = t.tokens();
            for fg in [k.fg, k.mut_, k.prose, k.focus, k.green, k.blue, k.yellow, k.magenta] {
                for bg in [k.bg, k.bar, k.sel] { assert!(contrast(fg, bg) >= 4.5, "{} {fg} on {bg}", t.name()); } }
            assert!(contrast(k.bd, k.bg) >= 3.0, "{} border", t.name()); } }
    #[test] fn contrast_reference() { assert!((contrast("#000000", "#ffffff") - 21.0).abs() < 0.01); }
    #[test] fn css_has_every_theme() { let c = css(); for t in Theme::ALL { assert!(c.contains(&format!("html[data-theme=\"{}\"]", t.name()))); } assert!(c.contains("--sel:#24304f")); }
}
