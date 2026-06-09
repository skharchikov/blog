# Blog Modernization Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Modernize the Leptos/Rust/WASM blog with a "Terminal++" visual language, put posts tag-filters in the URL, and make whole post cards clickable.

**Architecture:** CSS-only restyle (a design-token layer in `src/styles.css`) plus minimal, behavior-only Rust changes in four components. No `build.rs` or model changes. The posts card adopts the same whole-card-`<A>` + inner-element `stop_propagation` pattern that `projects.rs` already uses.

**Tech Stack:** Rust, Leptos 0.6 (CSR), leptos_router 0.6, Trunk, WASM, plain CSS (custom properties).

**Testing note:** This is a client-side WASM crate with no unit-test harness and no DOM test runner. "Tests" here are: `cargo check --target wasm32-unknown-unknown` (or plain `cargo check`) for compile correctness, `trunk build` for a full release build, and a final manual visual/interaction pass in the browser via `trunk serve`. Each task ends with a compile check and a commit; the final task is the visual verification.

**Branch:** Work happens on `ui/modernization` (already created; the design spec is committed there).

---

## File Structure

- `src/styles.css` — all visual changes: new token layer, shared card hover, Terminal++ post/project cards, home hero, nav underline, focus/reduced-motion rules. (Largest change, but CSS only.)
- `src/components/post_list.rs` — URL-driven tag filter, whole-card link, tag `stop_propagation`, read-time, counts, empty state.
- `src/components/home.rs` — `$ whoami` prompt, gradient caret markup, tagline + skill chips, reduced-motion-aware reveal.
- `src/components/nav.rs` — current-route underline class, `~` home glyph, reactive dark-mode toggle icon.
- `src/components/projects.rs` — markup unchanged except optional class hooks; restyle is CSS. (Touch only if a class hook is needed.)

---

## Task 1: Design token layer + shared card hover

**Files:**
- Modify: `src/styles.css` (`:root` block at lines 8-19, `body.dark-mode` block at lines 21-31, and append a new tokens/utility section)

- [ ] **Step 1: Add gradient + motion + spacing tokens to `:root`**

In `src/styles.css`, the `:root` block currently ends at line 18 (`--tag-text: #666666;`) before the closing `}` on line 19. Add these lines just before that closing `}`:

```css
    /* modernization tokens */
    --accent-grad: linear-gradient(135deg, #3b82f6, #8b5cf6);
    --ease: cubic-bezier(0.4, 0, 0.2, 1);
    --dur: 150ms;
    --radius: 12px;
    --radius-lg: 14px;
    --space-1: 0.25rem;
    --space-2: 0.5rem;
    --space-3: 0.75rem;
    --space-4: 1rem;
    --space-5: 1.5rem;
    --space-6: 2.5rem;
    --shadow-hover: 0 6px 20px rgba(0, 0, 0, 0.10);
    --card-rail: rgba(59, 130, 246, 0.0);
```

- [ ] **Step 2: Override shadow + gradient for dark mode**

In the `body.dark-mode` block (ends line 30 with `--tag-text: #a3a3a3;` before closing `}` on line 31), add before the closing `}`:

```css
    --accent-grad: linear-gradient(135deg, #3b82f6, #8b5cf6);
    --shadow-hover: 0 8px 24px rgba(0, 0, 0, 0.45);
```

- [ ] **Step 3: Append a shared hover utility at the end of the file**

Append to the end of `src/styles.css`:

```css
/* ===== Modernization: shared interaction primitives ===== */

/* Unified card hover used by post, project, and contact cards */
.post-card,
.project-card,
.contact-button {
    transition: transform var(--dur) var(--ease),
                border-color var(--dur) var(--ease),
                box-shadow var(--dur) var(--ease);
}

.post-card:hover,
.project-card:hover,
.contact-button:hover {
    border-color: var(--accent-color);
    transform: translateY(-2px);
    box-shadow: var(--shadow-hover);
}

/* Visible keyboard focus on everything interactive */
a:focus-visible,
button:focus-visible,
.post-card-link:focus-visible,
.project-card-link:focus-visible,
.tag-filter-btn:focus-visible,
.tag:focus-visible {
    outline: 2px solid var(--accent-color);
    outline-offset: 3px;
    border-radius: var(--radius);
}

@media (prefers-reduced-motion: reduce) {
    *,
    *::before,
    *::after {
        animation-duration: 0.001ms !important;
        animation-iteration-count: 1 !important;
        transition-duration: 0.001ms !important;
    }
    .post-card:hover,
    .project-card:hover,
    .contact-button:hover {
        transform: none;
    }
}
```

- [ ] **Step 4: Verify it builds**

Run: `cargo check`
Expected: PASS (CSS is not compiled by cargo, but this confirms nothing else broke; the real check is the build in Step 5).

Run: `trunk build`
Expected: build succeeds, `dist/` produced, no errors.

- [ ] **Step 5: Commit**

```bash
git add src/styles.css
git commit -m "$(cat <<'EOF'
Add design token layer + shared card hover

Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

## Task 2: URL-driven tag filter on posts

**Files:**
- Modify: `src/components/post_list.rs` (full rewrite of the component body)

Context: the current component (lines 1-93) holds the selected tag in a local `create_signal`. Replace that with the router query map so `/posts?tag=rust` is the source of truth.

- [ ] **Step 1: Replace the signal + filtering logic with query-driven logic**

Replace the entire contents of `src/components/post_list.rs` with:

```rust
use crate::models::BlogPost;
use leptos::*;
use leptos_router::*;

/// Estimate read time in minutes from raw post content (~200 wpm, min 1).
fn read_minutes(content: &str) -> usize {
    let words = content.split_whitespace().count();
    std::cmp::max(1, (words + 199) / 200)
}

#[component]
pub fn PostList() -> impl IntoView {
    let query = use_query_map();
    let navigate = use_navigate();

    let posts = BlogPost::all_posts();

    // Selected tag is derived from the URL (?tag=...), not local state.
    let selected_tag = move || query.with(|q| q.get("tag").cloned());

    let filtered_posts = move || {
        if let Some(tag) = selected_tag() {
            posts
                .iter()
                .filter(|post| post.tags.contains(&tag))
                .cloned()
                .collect::<Vec<_>>()
        } else {
            posts.to_vec()
        }
    };

    let all_tags = move || {
        let mut tags = std::collections::HashSet::new();
        for post in posts.iter() {
            for tag in &post.tags {
                tags.insert(tag.clone());
            }
        }
        let mut tag_vec: Vec<_> = tags.into_iter().collect();
        tag_vec.sort();
        tag_vec
    };

    // Count of posts carrying a given tag (for the "(n)" badge).
    let count_for = move |tag: &str| posts.iter().filter(|p| p.tags.contains(&tag.to_string())).count();

    // Navigate helpers: "All" clears the query, a tag sets ?tag=...
    let nav_all = {
        let navigate = navigate.clone();
        move || navigate("/posts", Default::default())
    };
    let nav_tag = {
        let navigate = navigate.clone();
        move |tag: &str| navigate(&format!("/posts?tag={}", tag), Default::default())
    };

    view! {
        <div class="post-list-container">
            <h1 class="page-title"><span class="prompt">"~/posts $ "</span>"ls"</h1>

            <div class="tag-filter">
                <button
                    class="tag-filter-btn"
                    class:active={move || selected_tag().is_none()}
                    attr:aria-pressed={move || if selected_tag().is_none() { "true" } else { "false" }}
                    on:click={
                        let nav_all = nav_all.clone();
                        move |_| nav_all()
                    }
                >
                    "all"
                </button>
                {move || all_tags().into_iter().map(|tag| {
                    let tag_active = tag.clone();
                    let tag_click = tag.clone();
                    let tag_label = tag.clone();
                    let n = count_for(&tag);
                    let nav_tag = nav_tag.clone();
                    view! {
                        <button
                            class="tag-filter-btn"
                            class:active={move || selected_tag().as_deref() == Some(&tag_active)}
                            attr:aria-pressed={
                                let t = tag_active.clone();
                                move || if selected_tag().as_deref() == Some(&t) { "true" } else { "false" }
                            }
                            on:click=move |_| nav_tag(&tag_click)
                        >
                            {format!("{} ({})", tag_label, n)}
                        </button>
                    }
                }).collect_view()}
            </div>

            <div class="post-grid">
                {move || {
                    let items = filtered_posts();
                    if items.is_empty() {
                        view! { <p class="post-empty">"No posts with this tag yet."</p> }.into_view()
                    } else {
                        items.into_iter().map(|post| {
                            let nav_tag = nav_tag.clone();
                            let minutes = read_minutes(&post.content);
                            view! {
                                <A href={format!("/posts/{}", post.slug)} class="post-card-link">
                                    <article class="post-card">
                                        <div class="post-card-header">
                                            <h2 class="post-card-title">{&post.title}</h2>
                                            <time class="post-date">{format!("{} · {} min", post.date, minutes)}</time>
                                        </div>
                                        <p class="post-excerpt">{&post.excerpt}</p>
                                        <div class="post-tags">
                                            {post.tags.iter().map(|tag| {
                                                let tag_click = tag.clone();
                                                let nav_tag = nav_tag.clone();
                                                view! {
                                                    <span
                                                        class="tag"
                                                        on:click=move |ev| {
                                                            ev.prevent_default();
                                                            ev.stop_propagation();
                                                            nav_tag(&tag_click);
                                                        }
                                                    >
                                                        {tag}
                                                    </span>
                                                }
                                            }).collect_view()}
                                        </div>
                                    </article>
                                </A>
                            }
                        }).collect_view().into_view()
                    }
                }}
            </div>
        </div>
    }
}
```

This single task covers spec sections 2 (URL filters, whole-card link, tag stop_propagation, counts, empty state) and the read-time meta from section 6 — they are one cohesive rewrite of the same file and cannot be split without leaving the file uncompilable.

- [ ] **Step 2: Verify it compiles**

Run: `cargo check`
Expected: PASS, no errors. If `use_navigate` returns a value that must be `Clone`d into closures, the `.clone()` calls above already handle it. If the compiler complains that `navigate` is `FnOnce`, confirm leptos_router 0.6's `use_navigate()` returns a `Clone` closure (it does) — no change needed.

- [ ] **Step 3: Build**

Run: `trunk build`
Expected: build succeeds.

- [ ] **Step 4: Commit**

```bash
git add src/components/post_list.rs
git commit -m "$(cat <<'EOF'
Drive post tag filter from URL; whole card clickable; add read-time

Filters now live in ?tag= so they are shareable and back-button aware.
Whole card is an <A>; tag pills stop propagation to filter instead of
navigating. Adds per-tag counts, empty state, and read-time meta.

Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

## Task 3: Terminal++ styling for post cards + filters

**Files:**
- Modify: `src/styles.css` (post-card section lines ~824-895, plus append new rules)

- [ ] **Step 1: Update the page-title prompt + post-card visuals**

Append to the end of `src/styles.css`:

```css
/* ===== Modernization: posts page ===== */

.page-title .prompt {
    color: var(--accent-color);
    font-weight: 600;
}

.post-grid {
    gap: var(--space-5); /* airier rhythm */
}

.post-card-link {
    text-decoration: none;
    color: inherit;
    display: block;
}

/* Accent gradient rail that fades in on hover */
.post-card {
    position: relative;
    overflow: hidden;
    padding-left: var(--space-5);
}

.post-card::before {
    content: "";
    position: absolute;
    left: 0;
    top: 0;
    bottom: 0;
    width: 3px;
    background: var(--accent-grad);
    opacity: 0;
    transition: opacity var(--dur) var(--ease);
}

.post-card-link:hover .post-card::before,
.post-card-link:focus-visible .post-card::before {
    opacity: 1;
}

.post-card-title {
    font-size: 1.375rem;
    font-weight: 600;
    letter-spacing: -0.01em;
    color: var(--text-primary);
    transition: color var(--dur) var(--ease);
}

.post-card-title::before {
    content: "$ ";
    color: var(--accent-color);
    font-weight: 700;
}

.post-card-link:hover .post-card-title {
    color: var(--accent-color);
}

.post-empty {
    color: var(--text-tertiary);
    font-size: 0.9375rem;
    padding: var(--space-4) 0;
}

/* Active filter pill uses the accent gradient */
.tag-filter-btn.active {
    background: var(--accent-grad);
    border-color: transparent;
    color: #fff;
}
```

Note: the old `.post-title-link` rules (lines ~848-859) are now unused because the title is no longer a separate `<A>`. Leaving them in place is harmless, but to keep CSS clean, delete the `.post-title-link` and `.post-title-link:hover` blocks (lines ~848-859 in the original file).

- [ ] **Step 2: Build**

Run: `trunk build`
Expected: build succeeds.

- [ ] **Step 3: Commit**

```bash
git add src/styles.css
git commit -m "$(cat <<'EOF'
Style posts page in Terminal++ (prompt title, card rail, gradient filter)

Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

## Task 4: Home hero — `$ whoami`, tagline, skill chips

**Files:**
- Modify: `src/components/home.rs` (view block, lines 50-74)
- Modify: `src/styles.css` (append home rules)

- [ ] **Step 1: Add the prompt line, gradient caret class, tagline, and chips**

In `src/components/home.rs`, replace the `view!` block (lines 50-74, from `view! {` to the closing `}` before the final `}`) with:

```rust
    view! {
        <div class="home-container">
            <div class="home-content">
                <div class="home-prompt" class:visible={move || !typed_text.get().is_empty()}>
                    "$ whoami"
                </div>
                <div class="home-logo-container">
                    <Programmer visible={show_github} />
                    <h1 class="home-logo">
                        {move || typed_text.get()}
                        <span
                            class="cursor gradient-caret"
                            class:slow-blink={move || cursor_state.get() == "slow-blink"}
                            class:stopped={move || cursor_state.get() == "stopped"}
                        ></span>
                    </h1>
                </div>
                <div class="home-tagline" class:visible={move || show_github.get()}>
                    <b>"Rust"</b>" · "<b>"Scala"</b>" · backend & distributed systems"
                </div>
                <div class="home-chips" class:visible={move || show_github.get()}>
                    <span class="chip hot">"Rust"</span>
                    <span class="chip">"Scala"</span>
                    <span class="chip">"async"</span>
                    <span class="chip">"low-latency"</span>
                    <span class="chip">"distributed"</span>
                </div>
            </div>
        </div>
    }
```

(The commented-out GitHub-button block from the original is dropped in this replacement; it was dead code.)

- [ ] **Step 2: Add home hero CSS**

Append to the end of `src/styles.css`:

```css
/* ===== Modernization: home hero ===== */

.home-prompt {
    font-family: 'IBM Plex Mono', monospace;
    color: var(--accent-color);
    font-size: 1.125rem;
    opacity: 0;
    transition: opacity var(--dur) var(--ease);
    margin-bottom: var(--space-2);
}

.home-prompt.visible {
    opacity: 1;
}

.home-logo .cursor.gradient-caret {
    background: var(--accent-grad);
    border-radius: 2px;
}

.home-tagline,
.home-chips {
    opacity: 0;
    transition: opacity 0.6s var(--ease);
}

.home-tagline.visible,
.home-chips.visible {
    opacity: 1;
}

.home-tagline {
    margin-top: var(--space-4);
    color: var(--text-secondary);
    font-size: 1rem;
}

.home-tagline b {
    color: var(--text-primary);
    font-weight: 600;
}

.home-chips {
    margin-top: var(--space-5);
    display: flex;
    gap: var(--space-2);
    flex-wrap: wrap;
    justify-content: center;
}

.chip {
    font-size: 0.8125rem;
    padding: var(--space-1) var(--space-3);
    border: 2px solid var(--border-color);
    border-radius: var(--radius);
    color: var(--text-secondary);
    background: var(--bg-secondary);
}

.chip.hot {
    border-color: transparent;
    background: var(--accent-grad);
    color: #fff;
}
```

- [ ] **Step 3: Build**

Run: `trunk build`
Expected: build succeeds.

- [ ] **Step 4: Commit**

```bash
git add src/components/home.rs src/styles.css
git commit -m "$(cat <<'EOF'
Add Terminal++ home hero: whoami prompt, gradient caret, tagline, chips

Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

## Task 5: Nav — current-route underline, home `~` glyph, reactive toggle icon

**Files:**
- Modify: `src/components/nav.rs` (full rewrite)
- Modify: `src/styles.css` (append nav rules)

- [ ] **Step 1: Rewrite `nav.rs` to add active-route class, home glyph, and reactive icon**

Replace the entire contents of `src/components/nav.rs` with:

```rust
use leptos::*;
use leptos_router::*;

#[component]
pub fn Nav() -> impl IntoView {
    let dark_mode = use_context::<ReadSignal<bool>>()
        .expect("dark mode ReadSignal not provided in App");
    let set_dark_mode = use_context::<WriteSignal<bool>>()
        .expect("dark mode WriteSignal not provided in App");

    let toggle_dark_mode = move |_| {
        set_dark_mode.update(|mode| *mode = !*mode);
    };

    let location = use_location();
    let is_home = move || location.pathname.get() == "/";

    let back_link = move || {
        let path = location.pathname.get();
        if path.starts_with("/projects/") {
            "/projects".to_string()
        } else if path.starts_with("/posts/") {
            "/posts".to_string()
        } else {
            "/".to_string()
        }
    };

    // Whether a top-level nav route is the current section.
    let active = move |prefix: &str| {
        let path = location.pathname.get();
        path == prefix || path.starts_with(&format!("{}/", prefix))
    };

    view! {
        <nav class="navbar">
            <div class="nav-content">
                <div class="nav-left">
                    {move || if is_home() {
                        view! { <span class="nav-home-glyph">"~"</span> }.into_view()
                    } else {
                        view! {
                            <A href={back_link} class="nav-link back-link">"← back"</A>
                        }.into_view()
                    }}
                </div>
                <div class="nav-center">
                    <A href="/projects" class=move || if active("/projects") { "nav-link current" } else { "nav-link" }>"projects"</A>
                    <A href="/posts" class=move || if active("/posts") { "nav-link current" } else { "nav-link" }>"posts"</A>
                    <A href="/contacts" class=move || if active("/contacts") { "nav-link current" } else { "nav-link" }>"contacts"</A>
                </div>
                <div class="nav-right">
                    <button
                        class="dark-mode-toggle"
                        aria-label="Toggle dark mode"
                        on:click=toggle_dark_mode
                    >
                        {move || if dark_mode.get() { "☾" } else { "☀" }}
                    </button>
                </div>
            </div>
        </nav>
    }
}
```

- [ ] **Step 2: Add nav underline + glyph CSS**

Append to the end of `src/styles.css`:

```css
/* ===== Modernization: nav ===== */

.nav-home-glyph {
    color: var(--accent-color);
    font-size: 1.25rem;
    font-weight: 600;
}

.nav-link {
    position: relative;
}

.nav-link.current {
    color: var(--text-primary);
}

.nav-link.current::after {
    content: "";
    position: absolute;
    left: 0;
    right: 0;
    bottom: -0.4rem;
    height: 2px;
    border-radius: 2px;
    background: var(--accent-grad);
}
```

- [ ] **Step 3: Build**

Run: `trunk build`
Expected: build succeeds.

Note: `App` already calls `provide_context(dark_mode)` (the `ReadSignal`) at `src/app.rs:17`, so `use_context::<ReadSignal<bool>>()` resolves. If it panics at runtime, confirm that line is present.

- [ ] **Step 4: Commit**

```bash
git add src/components/nav.rs src/styles.css
git commit -m "$(cat <<'EOF'
Nav: active-route underline, home ~ glyph, reactive dark-mode icon, aria-label

Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

## Task 6: Project & contact cards — unify with new hover + tag pills, a11y on GitHub icon

**Files:**
- Modify: `src/styles.css` (project tag pills + remove redundant project-card hover)
- Modify: `src/components/projects.rs` (add `aria-label` to the GitHub link)

- [ ] **Step 1: Remove the now-redundant project-card hover and add padding for rail consistency**

In `src/styles.css`, the `.project-card:hover` block (lines ~538-540) only sets `border-color`. The shared rule from Task 1 now handles hover. Delete the `.project-card:hover { border-color: var(--accent-color); }` block (lines 538-540) to avoid a duplicate/conflicting rule.

Then append to the end of `src/styles.css`:

```css
/* ===== Modernization: project + contact polish ===== */

.project-tags .tag,
.post-tags .tag {
    border-radius: var(--radius);
    transition: border-color var(--dur) var(--ease), color var(--dur) var(--ease);
}
```

- [ ] **Step 2: Add an aria-label to the project GitHub link**

In `src/components/projects.rs`, the GitHub anchor opens at line 20. Add an `aria-label` attribute. Change:

```rust
                                        class="project-github-link"
                                        on:click=move |ev| {
```

to:

```rust
                                        class="project-github-link"
                                        aria-label="View on GitHub"
                                        on:click=move |ev| {
```

- [ ] **Step 3: Build**

Run: `trunk build`
Expected: build succeeds.

- [ ] **Step 4: Commit**

```bash
git add src/styles.css src/components/projects.rs
git commit -m "$(cat <<'EOF'
Unify project/contact card hover with shared rule; label GitHub icon

Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

## Task 7: Final visual + interaction verification

**Files:** none (verification only)

- [ ] **Step 1: Serve the site**

Run: `trunk serve`
Expected: serves on `http://localhost:8080` (or the configured port) with no console errors.

- [ ] **Step 2: Verify the success criteria from the spec**

Check each in the browser (both light and dark mode — toggle in nav):

- [ ] Home shows `$ whoami`, name with gradient caret, then tagline + skill chips fade in.
- [ ] Nav current route has the gradient underline; `~` glyph shows on home, `← back` on subpages; toggle icon swaps `☀`/`☾`.
- [ ] `/posts` lists cards; hovering a card lifts it + shows the gradient rail; clicking anywhere on the card (not a tag) opens the post.
- [ ] Clicking a tag pill on a card filters and updates the URL to `/posts?tag=<tag>`.
- [ ] Loading `/posts?tag=rust` directly shows it filtered; the active filter pill has the gradient; counts like `rust (n)` show.
- [ ] Browser back button reverses filter changes.
- [ ] A tag with no posts (or manually visiting `/posts?tag=zzz`) shows the empty-state line.
- [ ] Card meta shows `date · N min`.
- [ ] Keyboard: Tab to a card and press Enter — it opens. Focus ring is visible.
- [ ] With OS "reduce motion" on, the typing animation and hover lifts are suppressed.

- [ ] **Step 3: If all pass, no commit needed (verification task).** If any fail, fix in the relevant file and amend/commit per that file's task.

---

## Self-Review

**Spec coverage check:**
- Tokens (gradient/ease/spacing/shadow) → Task 1 ✓
- Shared card hover across pages → Task 1 (rule) + Task 6 (remove dup) ✓
- URL filters (`?tag=`, navigate, back, counts, empty, aria-pressed) → Task 2 ✓
- Whole-card clickable + tag stop_propagation → Task 2 ✓
- Terminal++ post visuals (prompt title, rail, `$` prefix, spacing) → Task 3 ✓
- Home hero (`$ whoami`, gradient caret, tagline, chips, no bio) → Task 4 ✓
- Reduced-motion → Task 1 (global) + Task 4 (reveal degrades) ✓
- Nav underline + `~` glyph + reactive toggle + aria-label → Task 5 ✓
- Project/contact unify + GitHub aria-label → Task 6 ✓
- Read-time meta → Task 2 (`read_minutes`) ✓
- Focus-visible / aria → Task 1 + Task 5 + Task 6 ✓
- Out-of-scope items (og:image, progress bar, view-transitions, component extraction) → correctly absent ✓

**Type/name consistency:** `read_minutes` defined and used in Task 2 only. `selected_tag` is a closure (`move || Option<String>`) used consistently. CSS classes introduced (`prompt`, `post-card-title`, `post-card-link`, `post-empty`, `home-prompt`, `home-tagline`, `home-chips`, `chip`, `gradient-caret`, `nav-home-glyph`, `nav-link.current`) are each defined in CSS and referenced in markup. No dangling references.

**Placeholder scan:** No TBD/TODO; every code step has full code.
