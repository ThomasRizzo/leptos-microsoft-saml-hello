use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{window, Storage};

/// User information extracted from SAML assertion (or JWT claims in production).
/// Extend this struct with fields that match the claims you configure in Entra ID
/// (e.g. http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress, name, etc.)
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserInfo {
    pub display_name: String,
    pub email: String,
    pub upn: String,
    pub object_id: Option<String>,
}

/// Configuration - UPDATE THESE VALUES for your Microsoft Entra ID tenant/app
const TENANT_ID: &str = "YOUR_TENANT_ID_OR_DOMAIN"; // e.g. "contoso.onmicrosoft.com" or a GUID
const ENTITY_ID: &str = "urn:leptos-saml-demo:sp";   // Must match what you configured in Entra ID

#[component]
pub fn App() -> impl IntoView {
    // Reactive auth state
    let (user, set_user) = signal::<Option<UserInfo>>(None);
    let (auth_error, set_auth_error) = signal::<Option<String>>(None);
    let (is_loading, set_is_loading) = signal(false);

    // Check for existing session on mount (localStorage in this demo)
    Effect::new(move |_| {
        if let Some(storage) = get_local_storage() {
            if let Ok(Some(stored_json)) = storage.get_item("saml_demo_user") {
                // In a real app you'd validate a JWT here instead of trusting localStorage
                if let Some(parsed) = parse_mock_user(&stored_json) {
                    set_user.set(Some(parsed));
                }
            }
        }
    });

    // Initiate SP-initiated SAML login (or IdP-initiated via the Login URL)
    let start_saml_login = move |_| {
        set_is_loading.set(true);
        set_auth_error.set(None);

        // Build the Microsoft SAML endpoint.
        // For a full SP-initiated flow you would also generate a SAMLRequest (AuthnRequest XML,
        // deflate + base64). For this hello world we redirect to the tenant's SAML endpoint.
        // Many organizations use the "Login URL" shown in Entra ID SSO setup directly.
        let login_url = if TENANT_ID == "YOUR_TENANT_ID_OR_DOMAIN" {
            // Fallback demo URL (will show Microsoft login page but may not work without config)
            "https://login.microsoftonline.com/common/saml2".to_string()
        } else {
            format!("https://login.microsoftonline.com/{}/saml2", TENANT_ID)
        };

        if let Some(w) = window() {
            // In production you might want to set RelayState to return to current page or a specific route
            let _ = w.location().set_href(&login_url);
        }
        set_is_loading.set(false);
    };

    // Simulate a successful SAML callback (useful for demo without full backend)
    // In production this would be replaced by reading a JWT from URL/cookie after backend validation
    let simulate_successful_login = move |_| {
        set_is_loading.set(true);
        set_auth_error.set(None);

        let mock_user = UserInfo {
            display_name: "Thomas Rizzo".to_string(),
            email: "thomas.rizzo@contoso.com".to_string(),
            upn: "thomas.rizzo@contoso.com".to_string(),
            object_id: Some("00000000-0000-0000-0000-000000000000".to_string()),
        };

        // Persist in localStorage (demo only — use secure HttpOnly cookie or validated JWT in prod)
        if let Some(storage) = get_local_storage() {
            let json = format!(
                r#"{{"display_name":"{}","email":"{}","upn":"{}","object_id":"{}"}}"#,
                mock_user.display_name,
                mock_user.email,
                mock_user.upn,
                mock_user.object_id.as_deref().unwrap_or("")
            );
            let _ = storage.set_item("saml_demo_user", &json);
        }

        set_user.set(Some(mock_user));
        set_is_loading.set(false);
    };

    let logout = move |_| {
        if let Some(storage) = get_local_storage() {
            let _ = storage.remove_item("saml_demo_user");
        }
        set_user.set(None);
        set_auth_error.set(None);
        
        // Optional: Redirect to Microsoft logout for full SLO experience
        // let logout_url = format!("https://login.microsoftonline.com/{}/oauth2/v2.0/logout", TENANT_ID);
        // if let Some(w) = window() { let _ = w.location().set_href(&logout_url); }
    };

    // Clear any error when user interacts
    let clear_error = move |_| {
        set_auth_error.set(None);
    };

    view! {
        <div class="min-h-screen bg-zinc-950 flex flex-col">
            {/* Top nav */}
            <nav class="border-b border-zinc-800 bg-zinc-900/50 backdrop-blur-lg sticky top-0 z-50">
                <div class="max-w-5xl mx-auto px-6 h-16 flex items-center justify-between">
                    <div class="flex items-center gap-x-3">
                        <div class="w-9 h-9 bg-blue-600 rounded-2xl flex items-center justify-center shadow-inner">
                            <span class="text-white text-2xl font-bold tracking-tighter">L</span>
                        </div>
                        <div>
                            <div class="font-semibold text-xl tracking-tight">"Leptos SAML"</div>
                            <div class="text-[10px] text-zinc-500 -mt-1">"Hello World Demo"</div>
                        </div>
                    </div>
                    
                    <div class="flex items-center gap-x-2 text-sm">
                        <div class="px-3 py-1 rounded-full bg-zinc-900 border border-zinc-800 text-zinc-400 text-xs font-mono">
                            "Rust • WASM • Trunk"
                        </div>
                        {move || if user.get().is_some() {
                            view! { <div class="px-3 py-1 rounded-full bg-emerald-500/10 text-emerald-400 text-xs font-medium border border-emerald-500/20">"Authenticated"</div> }.into_any()
                        } else {
                            view! { <div></div> }.into_any()
                        }}
                    </div>
                </div>
            </nav>

            <div class="flex-1 flex items-center justify-center p-6">
                <div class="w-full max-w-lg">
                    {/* Header */}
                    <div class="text-center mb-10">
                        <div class="inline-flex items-center gap-x-2 px-4 py-1.5 rounded-3xl bg-zinc-900 border border-zinc-800 mb-6">
                            <div class="w-2 h-2 bg-emerald-400 rounded-full animate-pulse"></div>
                            <span class="text-xs font-medium tracking-[2px] text-zinc-400">"MICROSOFT ENTRA ID • SAML 2.0"</span>
                        </div>
                        
                        <h1 class="text-6xl font-semibold tracking-tighter text-white mb-3">
                            "Hello, World."
                        </h1>
                        <p class="text-xl text-zinc-400 max-w-sm mx-auto">
                            "A minimal Leptos + WASM app authenticating against your organization's Microsoft SAML."
                        </p>
                    </div>

                    {/* Main Card */}
                    <div class="bg-zinc-900 border border-zinc-800 rounded-3xl p-8 shadow-2xl">
                        {move || match (user.get(), is_loading.get()) {
                            (Some(u), false) => view! {
                                <div class="space-y-8">
                                    {/* User Info Section */}
                                    <div>
                                        <div class="flex items-center gap-x-4 mb-6">
                                            <div class="w-14 h-14 bg-gradient-to-br from-blue-500 to-indigo-600 rounded-2xl flex-shrink-0 flex items-center justify-center text-3xl shadow-inner">
                                                "👤"
                                            </div>
                                            <div class="min-w-0">
                                                <div class="font-semibold text-2xl tracking-tight text-white truncate">{u.display_name.clone()}</div>
                                                <div class="text-emerald-400 text-sm font-medium">"Signed in via SAML"</div>
                                            </div>
                                        </div>

                                        <div class="bg-zinc-950 border border-zinc-800 rounded-2xl p-5 space-y-4 text-sm">
                                            <div class="flex justify-between items-center py-1 border-b border-zinc-800">
                                                <span class="text-zinc-400">"Email / UPN"</span>
                                                <span class="font-mono text-white truncate max-w-[220px] text-right">{u.email.clone()}</span>
                                            </div>
                                            <div class="flex justify-between items-center py-1 border-b border-zinc-800">
                                                <span class="text-zinc-400">"User Principal Name"</span>
                                                <span class="font-mono text-white truncate max-w-[220px] text-right">{u.upn.clone()}</span>
                                            </div>
                                            {u.object_id.as_ref().map(|oid| view! {
                                                <div class="flex justify-between items-center py-1">
                                                    <span class="text-zinc-400">"Object ID"</span>
                                                    <span class="font-mono text-white text-xs truncate max-w-[220px] text-right">{oid.clone()}</span>
                                                </div>
                                            })}
                                        </div>
                                    </div>

                                    <button
                                        on:click=logout
                                        class="w-full py-4 text-base font-semibold bg-zinc-800 hover:bg-red-600/90 active:bg-red-700 transition-all rounded-2xl border border-zinc-700 hover:border-red-500/50 text-white flex items-center justify-center gap-x-2"
                                    >
                                        <span>"Sign out of Microsoft"</span>
                                    </button>
                                </div>
                            }.into_any(),

                            (None, false) => view! {
                                <div class="space-y-6">
                                    <div class="text-center py-4">
                                        <p class="text-zinc-400 text-[15px] leading-relaxed max-w-[300px] mx-auto">
                                            "Click below to start the SAML authentication flow with your Microsoft Entra ID tenant."
                                        </p>
                                    </div>

                                    <button
                                        on:click=start_saml_login
                                        disabled=move || TENANT_ID == "YOUR_TENANT_ID_OR_DOMAIN"
                                        class="w-full group py-5 text-lg font-semibold bg-white text-zinc-950 hover:bg-zinc-100 active:bg-zinc-200 transition-all rounded-2xl flex items-center justify-center gap-x-3 disabled:opacity-40 disabled:cursor-not-allowed shadow-xl"
                                    >
                                        <span class="text-xl">"🔐"</span>
                                        <span>"Continue with Microsoft"</span>
                                    </button>

                                    <div class="text-center">
                                        <button
                                            on:click=simulate_successful_login
                                            class="text-xs text-zinc-500 hover:text-zinc-300 underline underline-offset-4 transition-colors"
                                        >
                                            "Demo mode: Simulate successful SAML response"
                                        </button>
                                    </div>

                                    {move || if TENANT_ID == "YOUR_TENANT_ID_OR_DOMAIN" {
                                        view! {
                                            <div class="text-[10px] text-amber-400/70 text-center px-4">
                                                "⚠️ Update <code class=\"font-mono\">TENANT_ID</code> in src/lib.rs with your Entra tenant"
                                            </div>
                                        }.into_any()
                                    } else { view! { <div></div> }.into_any() }}
                                </div>
                            }.into_any(),

                            (_, true) => view! {
                                <div class="flex flex-col items-center justify-center py-12">
                                    <div class="w-8 h-8 border-2 border-white/30 border-t-white rounded-full animate-spin mb-4"></div>
                                    <p class="text-sm text-zinc-400">"Redirecting to Microsoft..."</p>
                                </div>
                            }.into_any()
                        }}

                        {/* Error display */}
                        {move || auth_error.get().map(|err| view! {
                            <div 
                                on:click=clear_error
                                class="mt-6 p-4 bg-red-950/60 border border-red-900/50 text-red-400 text-sm rounded-2xl cursor-pointer hover:bg-red-950/80 transition-colors"
                            >
                                {err}
                                <div class="text-[10px] mt-1 text-red-500/70">"Click to dismiss"</div>
                            </div>
                        })}
                    </div>

                    {/* Footer info */}
                    <div class="mt-8 text-center text-[10px] text-zinc-500 font-mono tracking-widest">
                        "CSR MODE • LEPTOS 0.8 • TRUNK • WASM32"
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Helper to get localStorage safely
fn get_local_storage() -> Option<Storage> {
    window()
        .and_then(|w| w.local_storage().ok())
        .flatten()
}

/// Very simple mock parser for the demo persisted JSON.
/// In production you would decode + validate a real JWT.
fn parse_mock_user(json: &str) -> Option<UserInfo> {
    // Extremely naive parsing for demo purposes only
    if json.contains("display_name") && json.contains("email") {
        Some(UserInfo {
            display_name: extract_json_string(json, "display_name").unwrap_or("Demo User".into()),
            email: extract_json_string(json, "email").unwrap_or("demo@example.com".into()),
            upn: extract_json_string(json, "upn").unwrap_or("demo@example.com".into()),
            object_id: extract_json_string(json, "object_id"),
        })
    } else {
        None
    }
}

fn extract_json_string(json: &str, key: &str) -> Option<String> {
    let pattern = format!("\"{}\":\"", key);
    if let Some(start) = json.find(&pattern) {
        let value_start = start + pattern.len();
        if let Some(end) = json[value_start..].find('"') {
            return Some(json[value_start..value_start + end].to_string());
        }
    }
    None
}

/// WASM entry point
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    
    // Mount the Leptos app to the body (or a specific element)
    mount_to_body(App);
}