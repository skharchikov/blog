# Blog Modernization — Design Spec

**Date:** 2026-06-08
**Status:** Approved (design), pending implementation plan

## Goal

Make the personal blog (Leptos CSR / Rust / WASM) look more modern and polished
to signal engineering skill to **recruiters and peer developers**, while keeping
the site simple and blog-content-friendly. Also resolve two pieces of frontend-dev
feedback:

1. Tag filters on the posts page are not part of the URL (not shareable, back
   button does nothing).
2. Hovering a post card highlights the whole card, but only the title is
   clickable — misleading affordance. (Projects cards are already fully
   clickable; posts diverge.)

## Visual Direction

Chosen direction: **"Terminal++"** card/identity language with the airy spacing
of an editorial layout. The site keeps its existing CLI / monospace soul
(IBM Plex Mono body, JetBrains Mono code) and light/dark themes, but adds:

- A restrained accent **gradient** (`#3b82f6 → #8b5cf6`) used sparingly: blinking
  caret, active filter pill, card hover rail, nav current-route underline,
  "hot" skill chip.
- A command-line motif: `~/posts $ ls` page title, `$ whoami` on home, `~`
  prompt glyph in nav.
- More vertical breathing room (editorial spacing) and a consistent card hover
  (lift + shadow + accent border) across the whole site.

Restraint is the point — the gradient and motif are accents, not decoration
everywhere.

## Approach

CSS-only restyle plus minimal, behavior-only Rust changes (chosen over a full
component refactor — YAGNI for a site this size). The one cross-cutting cleanup:
make the posts card use the same whole-card-link + inner-element
`stop_propagation` pattern that `projects.rs` already uses, so the two pages stay
consistent.

**Files touched:** `src/styles.css`, `src/components/post_list.rs`,
`src/components/home.rs`, `src/components/nav.rs`,
`src/components/projects.rs` (restyle only).
**Not touched:** `build.rs`, `src/models/*` (no struct or generation changes).

## Design Detail

### 1. Design tokens (`styles.css`)

Extend the existing `:root` (and `body.dark-mode`) with a token layer:

- `--accent-grad: linear-gradient(135deg, #3b82f6, #8b5cf6);`
- `--ease: cubic-bezier(.4, 0, .2, 1);` and `--dur: 150ms;`
- Spacing scale `--space-1`…`--space-6` = `0.25 / 0.5 / 0.75 / 1 / 1.5 / 2.5rem`.
- `--radius: 12px;` `--radius-lg: 14px;`
- `--shadow-hover` tuned per theme (stronger in dark, softer in light).

Rules:
- Replace ad-hoc `0.1s` / `0.2s` / `all` transitions with
  `var(--dur) var(--ease)`.
- Single shared card-hover treatment: `translateY(-2px)` + `--shadow-hover` +
  accent border. Applied to post cards, project cards, contact buttons.
- Both themes must keep working; gradient/shadow values tuned per theme.

### 2. Posts page (`post_list.rs` + CSS) — carries both FE fixes

**URL filters:**
- Remove the local `selected_tag` signal. Derive the selected tag from the URL
  via `use_query_map()` reading `?tag=<value>`.
- `filtered_posts` reacts to the query map.
- Clicking a filter button or a tag pill calls `use_navigate()` to `/posts`
  (All) or `/posts?tag=rust`. Back button and shareable/bookmarkable links work
  with no extra code.
- "All" = no query param. Active filter styled with the accent gradient and
  carries `aria-pressed`.
- Filter buttons show counts, e.g. `rust (2)`.
- Empty state: a short line when zero posts match the active tag.

**Whole card clickable:**
- Wrap each card in `<A href="/posts/{slug}">` (mirrors `projects.rs:16`). The
  whole card becomes the link: pointer cursor + hover lift.
- Inner tag pills still set the filter, so their click handler must call
  `ev.prevent_default()` and `ev.stop_propagation()` to filter instead of
  navigating to the post — the same pattern `projects.rs:25` uses for the GitHub
  icon.
- Outcome: posts and projects behave identically.

**Terminal++ card visuals:**
- Page title rendered as `~/posts $ ls` with an accent-colored prompt.
- Card: accent gradient left-rail fades in on hover; `$ ` prefix on the title;
  meta line `date · N min`; tag pills below.
- Editorial spacing: larger vertical rhythm and gap between cards.

### 3. Home / landing (`home.rs` + CSS)

Extend the Terminal++ motif (keeps the existing typing animation):

- `$ whoami` prompt line above the name (accent color).
- Name `skharchikov` with a gradient blinking caret (reuse existing typing/blink
  state machine; restyle the caret to use the gradient).
- After typing completes (existing `show_github` trigger repurposed): fade in a
  tagline and skill chips.
- **Tagline:** `Rust · Scala · backend & distributed systems`.
- **Skill chips:** `Rust` (gradient / "hot"), `Scala`, `async`, `low-latency`,
  `distributed` — subtle pills.
- **No bio paragraph** (explicitly excluded for now).
- `prefers-reduced-motion`: skip typing, render the final state immediately.

### 4. Nav + global polish (`nav.rs`, `projects.rs`, CSS)

- **Nav:** gradient underline on the current route (active detection via the
  existing `use_location`). Keep the `← back` behavior on subpages, restyled; a
  `~` prompt glyph occupies the left slot on the home route. Dark-mode toggle
  swaps `☀` / `☾` based on state (currently a static `☀`).
- **Projects / contacts:** inherit the new shared card hover and tag-pill styles
  so the whole site feels unified. Projects cards are already fully clickable —
  restyle only.
- **Footer:** unchanged ("Built with Leptos 🦀").

### 5. Accessibility

- `:focus-visible` accent ring on all links, buttons, and cards.
- `prefers-reduced-motion` disables the typing animation, hover-lift transforms,
  and caret blink.
- `aria-pressed` on filter buttons; `aria-label` on icon-only buttons (dark-mode
  toggle, GitHub icon).
- Cards are `<A>` elements, so keyboard Enter activation works natively.

### 6. Read-time

- Compute at runtime from `post.content` word count ÷ ~200 wpm, `max(1)` minute.
  Small helper function in the posts component; no struct or `build.rs` change.
- Displayed in the card meta line as `date · N min`.

## Out of Scope (parked for later)

- Per-post `og:image` for richer link previews.
- Reading-progress bar on the post view.
- Route view-transitions / page-fade animations.
- Component extraction / shared `Card`/`Tag` components (full refactor).

## Success Criteria

- `/posts?tag=rust` filters on load, is shareable, and the browser back button
  reverses filter changes.
- Clicking anywhere on a post card (except a tag pill) opens the post; clicking
  a tag pill filters. Posts and projects behave identically.
- Home shows the `$ whoami` hero, gradient caret, tagline, and skill chips, and
  degrades gracefully with reduced-motion.
- Light and dark themes both render correctly with the new tokens.
- Keyboard and focus states work on all interactive elements.
- `trunk build` (release) succeeds; no console errors.
