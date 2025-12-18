#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlogPost {
    pub id: u32,
    pub title: String,
    pub slug: String,
    pub date: String,
    pub excerpt: String,
    pub content: String,
    pub tags: Vec<String>,
}

impl BlogPost {
    pub fn all_posts() -> Vec<Self> {
        vec![
            BlogPost {
                id: 1,
                title: "Getting Started with Leptos".to_string(),
                slug: "getting-started-with-leptos".to_string(),
                date: "2024-12-18".to_string(),
                excerpt: "An introduction to building reactive web applications with Leptos".to_string(),
                content: r#"# Getting Started with Leptos

Leptos is a cutting-edge Rust web framework that brings the power of fine-grained reactivity to the web. It compiles to WebAssembly and provides a modern development experience.

## Why Leptos?

- **Performance**: Compiles to WebAssembly for near-native speed
- **Reactivity**: Fine-grained reactive system similar to SolidJS
- **Type Safety**: Full Rust type safety in your frontend code
- **Modern DX**: Great developer experience with hot reloading

## Getting Started

To start with Leptos, you'll need to install Trunk, which is the build tool for Rust WASM applications:

```bash
cargo install trunk
```

Then create a new project and add Leptos as a dependency. You can start building reactive UIs that are both fast and type-safe!

## Next Steps

Check out the official Leptos documentation for more examples and guides. The community is friendly and growing rapidly."#.to_string(),
                tags: vec!["rust".to_string(), "leptos".to_string(), "webassembly".to_string()],
            },
            BlogPost {
                id: 2,
                title: "Rust Web Development in 2024".to_string(),
                slug: "rust-web-development-2024".to_string(),
                date: "2024-12-15".to_string(),
                excerpt: "Exploring the current state of Rust web frameworks and tooling".to_string(),
                content: r#"# Rust Web Development in 2024

The Rust web ecosystem has matured significantly in recent years. What was once a fragmented landscape has evolved into a rich ecosystem with multiple excellent choices.

## Popular Frameworks

1. **Leptos** - Reactive UI framework with CSR and SSR support
2. **Yew** - Component-based framework inspired by React
3. **Dioxus** - Cross-platform UI framework
4. **Axum** - Backend web framework built on Tokio

## Tooling

The Rust web development experience has improved dramatically with tools like:

- **Trunk**: Asset bundling and WASM building
- **wasm-pack**: Building and publishing WASM packages
- **cargo-leptos**: Leptos-specific build tool

## The Future

Rust's web ecosystem continues to grow. With WebAssembly becoming more mainstream and Rust's memory safety guarantees, we're seeing more companies adopt Rust for web development.

The future looks bright for Rust on the web!"#.to_string(),
                tags: vec!["rust".to_string(), "web".to_string()],
            },
            BlogPost {
                id: 3,
                title: "Understanding WebAssembly".to_string(),
                slug: "understanding-webassembly".to_string(),
                date: "2024-12-10".to_string(),
                excerpt: "A deep dive into WebAssembly and its role in modern web development".to_string(),
                content: r#"# Understanding WebAssembly

WebAssembly (WASM) is a binary instruction format that runs in web browsers at near-native speed. It's a game-changer for web performance.

## What is WebAssembly?

WebAssembly is a low-level bytecode format that can be executed at near-native speed in modern browsers. It's designed to be a compilation target for languages like Rust, C++, and Go.

## Benefits of WASM

- **Performance**: Near-native execution speed
- **Portability**: Run the same code across different platforms
- **Security**: Sandboxed execution environment
- **Interoperability**: Works alongside JavaScript

## Use Cases

WebAssembly excels in compute-intensive applications:

- Game engines and graphics
- Video and audio processing
- Scientific simulations
- Cryptography
- Image manipulation

## WASM and Rust

Rust has first-class support for WebAssembly. The combination of Rust's performance and safety with WASM's portability makes it an excellent choice for web applications.

With tools like wasm-bindgen, you can easily interact between Rust and JavaScript code, getting the best of both worlds."#.to_string(),
                tags: vec!["webassembly".to_string(), "web".to_string()],
            },
        ]
    }

    pub fn find_by_slug(slug: &str) -> Option<Self> {
        Self::all_posts().into_iter().find(|post| post.slug == slug)
    }
}
