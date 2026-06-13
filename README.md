# Leptos + Jama Connect REST API Explorer

A clean, modern Leptos 0.8 (CSR + WASM) demo that lets you:

- Paste an existing bearer token, **or**
- Enter Client ID + Client Secret to automatically fetch a fresh OAuth token from Jama
- Fetch and display your list of projects from the Jama REST API

## Features

- Two authentication modes in one UI
- Clean Tailwind + heroicons styling
- Proper error handling and loading states
- Works entirely in the browser (client-side)

## Quick Start

```bash
# 1. Install tools (first time only)
just install-tools

# 2. Run the dev server
just serve
```

Then open http://localhost:3000

## How to get your Jama credentials

### Option 1: Use an existing token (easiest for testing)
1. Log into Jama Connect
2. Go to your profile → **Set API Credentials** (or Admin area)
3. Generate or copy an access token if available, or use a session token for testing.

### Option 2: Client Credentials flow (recommended)
1. In Jama, go to your user profile → **Set API Credentials**
2. Create a new set of **Client ID + Client Secret**
3. Use those in the "Get Token" tab of this app

The app will POST to `/rest/oauth/token` using the standard OAuth2 Client Credentials grant.

## Important Notes

**CORS**: Because this is a pure client-side WASM app, your browser makes direct calls to your Jama instance. 

- If your Jama is on `jamacloud.com` or properly configured, it may work.
- Many on-prem or strict instances block browser CORS requests.
- For production/internal tools, the recommended pattern is to add a small Rust backend (Axum) that proxies the Jama calls and handles authentication.

This demo is intentionally simple so you can quickly test the API and then extend it.

## Project Structure

```
├── Cargo.toml
├── Trunk.toml
├── index.html
├── justfile
├── README.md
└── src/
    └── lib.rs          # All the Leptos UI + API logic
```

## Available `just` commands

```bash
just                    # Show all recipes
just install-tools      # Install trunk + wasm target
just serve              # Start dev server with hot reload
just build              # Production build to dist/
just check
just fmt
just clean
```

## Next Steps / Ideas

- Add more endpoints (items, relationships, test runs, etc.)
- Add token refresh logic
- Persist Client ID/Secret in localStorage (optional)
- Add a small Axum backend proxy for CORS + token security
- Switch to `leptos_router` for multiple pages

Built with ❤️ using Leptos, Rust, and Tailwind.
