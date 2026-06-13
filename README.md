# Leptos + Trunk + Microsoft SAML Hello World

A minimal "Hello World" sample application demonstrating:

- **Leptos** reactive UI compiled to WebAssembly (WASM)
- **Trunk** for building/bundling/serving the CSR (Client-Side Rendered) app
- **Microsoft Entra ID (Azure AD) SAML 2.0** authentication flow (SP-initiated redirect)
- Display of user information after authentication

This is a **frontend-only demo**. For production use with real SAML, you should add a thin Rust backend (e.g. Axum) to securely handle the SAML Assertion Consumer Service (ACS) endpoint, validate the SAMLResponse (signatures, conditions, replay protection), and issue a secure session token (JWT) that the Leptos app consumes.

## Prerequisites

- Rust (stable or nightly recommended for Leptos)
- `cargo install trunk`
- `rustup target add wasm32-unknown-unknown`
- (Optional) `cargo install leptosfmt` for formatting

## Quick Start

```bash
cd leptos-microsoft-saml-hello

# Development server with hot reload
trunk serve --port 3000 --open
```

Then open http://localhost:3000

## Project Structure

```
leptos-microsoft-saml-hello/
├── Cargo.toml
├── Trunk.toml
├── index.html
├── README.md
└── src/
    └── lib.rs
```

## How the Auth Flow Works in This Sample

1. **Login button** redirects the browser to your Microsoft Entra ID SAML endpoint (`https://login.microsoftonline.com/{tenant}/saml2`).

2. User authenticates at Microsoft.

3. Microsoft POSTs the `SAMLResponse` (base64 encoded assertion) to your configured **Reply URL (ACS)**.

4. **In this pure WASM demo**: The simulate button sets a mock user. In a real setup you would:
   - Have a backend receive the POST at e.g. `/saml/acs`
   - Validate the assertion using a SAML library (or delegate)
   - Create a short-lived JWT or session
   - Redirect back to the SPA with the token (e.g. `/#token=xxx` or query param, or HttpOnly cookie)
   - Leptos app reads the token on load and fetches user info (or decodes claims)

## Configuring Microsoft Entra ID (SAML)

1. Microsoft Entra admin center → **Enterprise applications** → **New application** → **Create your own application**
   - Name: `Leptos SAML Demo`
   - Integration: Non-gallery

2. Go to the app → **Single sign-on** → **SAML**

3. **Basic SAML Configuration**:
   - **Identifier (Entity ID)**: `urn:leptos-saml-demo:sp` (or your domain)
   - **Reply URL (Assertion Consumer Service URL)**: `https://your-app.example.com/` (or `https://localhost:3000/` for dev — note browsers block mixed content; use HTTPS in prod or tunnel)
   - **Sign on URL**: `https://your-app.example.com/`

4. **Attributes & Claims** (default is usually fine):
   - `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/nameidentifier` → User.ObjectID or UPN
   - Add `emailaddress`, `givenname`, `surname`, `upn` etc. as needed

5. Note the **Login URL** and **Azure AD Identifier** from the setup screen. Update the constants in `src/lib.rs`.

6. **Users and groups** → Assign yourself or test users.

7. For production, upload your app's certificate if you want signed AuthnRequests, and configure the IdP certificate for response validation (done on backend).

## Important Production Notes

- **Never validate SAML assertions in the browser.** The signature verification, audience check, NotOnOrAfter, etc. must happen server-side.
- Use **HTTP-Only, Secure cookies** or short-lived JWTs for session management.
- Consider using **OIDC** (Authorization Code + PKCE) instead of SAML for pure SPAs — Microsoft strongly supports it and there are simpler flows (though you asked for SAML).
- For GovCloud / GCC High (common in your environment), the endpoints are `login.microsoftonline.us` or specific GCC URLs.
- Add proper error handling, token refresh, and logout (SAML SLO is complex; often just local logout + redirect to Microsoft logout URL).

## Extending This Sample

- Add `leptos_router` for proper `/login`, `/callback` routes.
- Add a real backend with `axum` + a SAML crate (e.g. community `saml2` or implement basic validation).
- Persist a real token from your backend instead of the mock `UserInfo`.
- Use `web-sys` + `js_sys` to read `SAMLResponse` if you proxy the ACS through a service worker or small server that injects it.
- Style with real Tailwind (add `tailwind.config.js` and trunk pipeline) instead of the CDN used here for demo.

## License / Usage

Free to use and adapt for your internal tools. Customize the `UserInfo` struct to match the claims you map in Entra ID.

Happy coding with Leptos + Rust WASM! 🦀