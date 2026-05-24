# Web Danar - Personal Portfolio

Personal portfolio website built with **Rust** and **Leptos**.

This project is mainly used to showcase my profile, skills, projects, and contact information in a simple, fast, and lightweight web app.

The application currently uses **Server-Side Rendering (SSR)** with client-side hydration, so the initial page can be rendered from the server while still becoming interactive in the browser.

## Tech Stack

- Rust 2024
- Leptos 0.8.19
- Server-Side Rendering (SSR)
- Hydration
- Axum
- SeaORM
- PostgreSQL / Supabase
- SCSS / Sass
- Leptos Icons
- cargo-leptos
- WebAssembly (WASM)

## Project Structure

The current project structure is intentionally simple and may evolve as the application grows.

```text
src/
├── main.rs
├── lib.rs
├── app.rs
└── component/

style/
├── main.scss
└── styles/
