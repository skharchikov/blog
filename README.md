# Blog

A blog application built with [Leptos](https://leptos.dev/), a reactive Rust web framework that compiles to WebAssembly. You can see it's running at [skh.rs](https://skh.rs) 

## Features

- Blog post listing and navigation
- Dark mode with persistent preferences
- Client-side routing
- Responsive design

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Trunk](https://trunkrs.dev/): `cargo install trunk`

## Usage

**Development:**
```bash
trunk serve --open
```

**Production build:**
```bash
trunk build --release
```

## Routes

- `/` - Blog post list
- `/about` - About page
- `/post/:slug` - Individual post view

## Adding Posts

Edit `src/models/post.rs` and add a new `BlogPost` entry to the `all_posts()` function.
