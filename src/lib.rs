use leptos::*;
use leptos::ev::event_target_value;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use wasm_bindgen_futures::spawn_local;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Project {
    id: Option<i64>,
    projectKey: Option<String>,
    name: Option<String>,
    description: Option<String>,
    status: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct ProjectsResponse {
    data: Vec<Project>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type: Option<String>,
    expires_in: Option<i64>,
}

#[derive(Clone, Copy, PartialEq)]
enum Tab {
    UseToken,
    GetToken,
}

#[component]
pub fn App() -> impl IntoView {
    // State
    let (current_tab, set_current_tab) = create_signal(Tab::UseToken);
    let (base_url, set_base_url) = create_signal(String::from("https://yourcompany.jamacloud.com"));
    let (token, set_token) = create_signal(String::new());
    let (client_id, set_client_id) = create_signal(String::new());
    let (client_secret, set_client_secret) = create_signal(String::new());
    
    let (projects, set_projects) = create_signal(Vec::<Project>::new());
    let (is_loading, set_is_loading) = create_signal(false);
    let (error, set_error) = create_signal(Option::<String>::None);
    let (success_msg, set_success_msg) = create_signal(Option::<String>::None);

    let has_token = move || !token.get().trim().is_empty();

    // Fetch projects using current token
    let fetch_projects = move |_| {
        let base = base_url.get().trim_end_matches('/').to_string();
        let auth_token = token.get();

        if auth_token.trim().is_empty() {
            set_error.set(Some("Please enter a bearer token first.".to_string()));
            return;
        }

        set_is_loading.set(true);
        set_error.set(None);
        set_success_msg.set(None);
        set_projects.set(vec![]);

        spawn_local(async move {
            let url = format!("{}/rest/v1/projects", base);

            match Request::get(&url)
                .header("Authorization", &format!("Bearer {}", auth_token))
                .header("Accept", "application/json")
                .send()
                .await
            {
                Ok(resp) => {
                    if resp.ok() {
                        match resp.json::<ProjectsResponse>().await {
                            Ok(data) => {
                                set_projects.set(data.data);
                                set_success_msg.set(Some(format!("Loaded {} projects", data.data.len())));
                            }
                            Err(e) => {
                                set_error.set(Some(format!("Failed to parse projects: {}", e)));
                            }
                        }
                    } else {
                        let status = resp.status();
                        let text = resp.text().await.unwrap_or_default();
                        set_error.set(Some(format!("Jama API error ({}): {}", status, text)));
                    }
                }
                Err(e) => {
                    set_error.set(Some(format!("Network error: {}. Make sure the Jama URL is correct and CORS is allowed.", e)));
                }
            }
            set_is_loading.set(false);
        });
    };

    // Get token using Client Credentials
    let get_token = move |_| {
        let base = base_url.get().trim_end_matches('/').to_string();
        let cid = client_id.get();
        let secret = client_secret.get();

        if cid.trim().is_empty() || secret.trim().is_empty() {
            set_error.set(Some("Please enter both Client ID and Client Secret.".to_string()));
            return;
        }

        set_is_loading.set(true);
        set_error.set(None);
        set_success_msg.set(None);

        spawn_local(async move {
            let url = format!("{}/rest/oauth/token", base);
            let basic = format!("{}:{}", cid, secret);
            let basic_b64 = BASE64.encode(basic.as_bytes());

            let body = "grant_type=client_credentials";

            match Request::post(&url)
                .header("Authorization", &format!("Basic {}", basic_b64))
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(body)
                .send()
                .await
            {
                Ok(resp) => {
                    if resp.ok() {
                        match resp.json::<TokenResponse>().await {
                            Ok(token_resp) => {
                                set_token.set(token_resp.access_token.clone());
                                set_success_msg.set(Some("Successfully obtained access token!".to_string()));
                                // Switch to Use Token tab and show projects hint
                                set_current_tab.set(Tab::UseToken);
                            }
                            Err(e) => {
                                set_error.set(Some(format!("Failed to parse token response: {}", e)));
                            }
                        }
                    } else {
                        let status = resp.status();
                        let text = resp.text().await.unwrap_or_default();
                        set_error.set(Some(format!("Token request failed ({}): {}", status, text)));
                    }
                }
                Err(e) => {
                    set_error.set(Some(format!("Network error getting token: {}. Check your Jama URL.", e)));
                }
            }
            set_is_loading.set(false);
        });
    };

    let clear_all = move |_| {
        set_token.set(String::new());
        set_projects.set(vec![]);
        set_error.set(None);
        set_success_msg.set(None);
        set_client_id.set(String::new());
        set_client_secret.set(String::new());
    };

    view! {
        <div class="min-h-screen bg-slate-50">
            // Top nav
            <nav class="border-b bg-white">
                <div class="max-w-5xl mx-auto px-6 py-4 flex items-center justify-between">
                    <div class="flex items-center gap-x-3">
                        <div class="w-10 h-10 bg-blue-600 rounded-xl flex items-center justify-center">
                            <i class="fa-solid fa-plug text-white text-2xl"></i>
                        </div>
                        <div>
                            <h1 class="font-display text-2xl tracking-tight text-slate-900">Jama Connect</h1>
                            <p class="text-xs text-slate-500 -mt-1">Explorer • Leptos + WASM</p>
                        </div>
                    </div>
                    
                    <div class="flex items-center gap-x-2 text-sm">
                        <div class="px-3 py-1.5 bg-slate-100 rounded-full text-slate-600 flex items-center gap-x-2">
                            <i class="fa-solid fa-globe fa-sm"></i>
                            <span class="font-mono text-xs">{move || base_url.get()}</span>
                        </div>
                        {move || if has_token() {
                            view! { <div class="px-3 py-1 bg-emerald-100 text-emerald-700 rounded-full text-xs font-medium flex items-center gap-x-1.5">
                                <i class="fa-solid fa-check-circle"></i>
                                <span>Authenticated</span>
                            </div> }
                        } else {
                            view! { <div></div> }
                        }}
                    </div>
                </div>
            </nav>

            <div class="max-w-5xl mx-auto px-6 py-8">
                // Header
                <div class="mb-8">
                    <h2 class="text-3xl font-semibold tracking-tight text-slate-900">REST API Explorer</h2>
                    <p class="mt-2 text-slate-600 max-w-md">
                        Connect to your Jama instance and explore projects using OAuth or a bearer token.
                    </p>
                </div>

                // Authentication Card
                <div class="leptos-card bg-white border border-slate-200 rounded-3xl shadow-sm overflow-hidden mb-8">
                    <div class="px-6 pt-6 pb-4 border-b flex items-center justify-between bg-slate-50/50">
                        <div class="section-header">Authentication</div>
                        
                        <button 
                            on:click=clear_all
                            class="text-xs px-3 py-1.5 hover:bg-slate-100 rounded-xl text-slate-500 hover:text-slate-700 flex items-center gap-x-2 transition-colors"
                        >
                            <i class="fa-solid fa-undo fa-sm"></i>
                            <span>Clear</span>
                        </button>
                    </div>

                    // Tabs
                    <div class="flex border-b px-6">
                        <button 
                            class=move || format!("px-5 py-3 text-sm font-medium transition-colors {}", 
                                if current_tab.get() == Tab::UseToken { "tab-active text-blue-700" } else { "text-slate-500 hover:text-slate-700" })
                            on:click=move |_| set_current_tab.set(Tab::UseToken)
                        >
                            <i class="fa-solid fa-key mr-2"></i>
                            Use Existing Token
                        </button>
                        <button 
                            class=move || format!("px-5 py-3 text-sm font-medium transition-colors {}", 
                                if current_tab.get() == Tab::GetToken { "tab-active text-blue-700" } else { "text-slate-500 hover:text-slate-700" })
                            on:click=move |_| set_current_tab.set(Tab::GetToken)
                        >
                            <i class="fa-solid fa-magic mr-2"></i>
                            Get Token (Client ID + Secret)
                        </button>
                    </div>

                    <div class="p-6">
                        // Base URL (common to both)
                        <div class="mb-5">
                            <label class="block text-xs font-semibold tracking-wider text-slate-500 mb-1.5">JAMA BASE URL</label>
                            <input 
                                type="text"
                                class="w-full px-4 py-3 border border-slate-300 rounded-2xl text-sm focus:outline-none focus:border-blue-500 font-mono"
                                placeholder="https://yourcompany.jamacloud.com"
                                prop:value=base_url
                                on:input=move |ev| set_base_url.set(event_target_value(&ev))
                            />
                            <p class="mt-1.5 text-[10px] text-slate-400">Include https:// and no trailing slash</p>
                        </div>

                        // Tab content
                        {move || match current_tab.get() {
                            Tab::UseToken => view! {
                                <div>
                                    <div>
                                        <label class="block text-xs font-semibold tracking-wider text-slate-500 mb-1.5">BEARER TOKEN</label>
                                        <input 
                                            type="password"
                                            class="w-full px-4 py-3 border border-slate-300 rounded-2xl text-sm focus:outline-none focus:border-blue-500 font-mono"
                                            placeholder="Paste your Jama access token here"
                                            prop:value=token
                                            on:input=move |ev| set_token.set(event_target_value(&ev))
                                        />
                                    </div>
                                </div>
                            }.into_view(),
                            
                            Tab::GetToken => view! {
                                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                    <div>
                                        <label class="block text-xs font-semibold tracking-wider text-slate-500 mb-1.5">CLIENT ID</label>
                                        <input 
                                            type="text"
                                            class="w-full px-4 py-3 border border-slate-300 rounded-2xl text-sm focus:outline-none focus:border-blue-500 font-mono"
                                            placeholder="your-client-id"
                                            prop:value=client_id
                                            on:input=move |ev| set_client_id.set(event_target_value(&ev))
                                        />
                                    </div>
                                    <div>
                                        <label class="block text-xs font-semibold tracking-wider text-slate-500 mb-1.5">CLIENT SECRET</label>
                                        <input 
                                            type="password"
                                            class="w-full px-4 py-3 border border-slate-300 rounded-2xl text-sm focus:outline-none focus:border-blue-500 font-mono"
                                            placeholder="••••••••••••"
                                            prop:value=client_secret
                                            on:input=move |ev| set_client_secret.set(event_target_value(&ev))
                                        />
                                    </div>
                                </div>
                            }.into_view(),
                        }}

                        // Action buttons
                        <div class="mt-6 flex items-center gap-x-3">
                            {move || if current_tab.get() == Tab::GetToken {
                                view! {
                                    <button 
                                        on:click=get_token
                                        disabled=move || is_loading.get()
                                        class="flex-1 md:flex-none px-8 py-3.5 bg-blue-600 hover:bg-blue-700 disabled:bg-blue-400 transition-colors text-white rounded-2xl font-semibold text-sm flex items-center justify-center gap-x-2 shadow-sm"
                                    >
                                        {move || if is_loading.get() { 
                                            view! { <><i class="fa-solid fa-spinner fa-spin mr-2"></i> Getting Token... </> }
                                        } else { 
                                            view! { <><i class="fa-solid fa-key mr-2"></i> Get Access Token </> }
                                        }}
                                    </button>
                                }.into_view()
                            } else {
                                view! { <div></div> }.into_view()
                            }}

                            <button 
                                on:click=fetch_projects
                                disabled=move || is_loading.get() || !has_token()
                                class="flex-1 md:flex-none px-8 py-3.5 bg-emerald-600 hover:bg-emerald-700 disabled:bg-emerald-300 transition-colors text-white rounded-2xl font-semibold text-sm flex items-center justify-center gap-x-2 shadow-sm"
                            >
                                {move || if is_loading.get() { 
                                    view! { <><i class="fa-solid fa-spinner fa-spin mr-2"></i> Loading... </> }
                                } else { 
                                    view! { <><i class="fa-solid fa-sync mr-2"></i> Fetch Projects </> }
                                }}
                            </button>
                        </div>
                    </div>
                </div>

                // Status / Error / Success
                {move || error.get().map(|e| view! {
                    <div class="mb-6 px-5 py-4 bg-red-50 border border-red-200 text-red-700 rounded-2xl flex gap-x-3 text-sm">
                        <i class="fa-solid fa-exclamation-triangle mt-0.5"></i>
                        <div>{e}</div>
                    </div>
                })}

                {move || success_msg.get().map(|msg| view! {
                    <div class="mb-6 px-5 py-4 bg-emerald-50 border border-emerald-200 text-emerald-700 rounded-2xl flex gap-x-3 text-sm">
                        <i class="fa-solid fa-check-circle mt-0.5"></i>
                        <div>{msg}</div>
                    </div>
                })}

                // Projects section
                {move || if !projects.get().is_empty() {
                    view! {
                        <div class="leptos-card bg-white border border-slate-200 rounded-3xl shadow-sm overflow-hidden">
                            <div class="px-6 py-4 border-b flex items-center justify-between bg-slate-50/50">
                                <div>
                                    <div class="section-header">Projects</div>
                                    <div class="text-xs text-slate-400 mt-0.5">{projects.get().len()} projects found</div>
                                </div>
                                <button 
                                    on:click=fetch_projects
                                    class="text-xs px-4 py-2 hover:bg-white border border-slate-200 rounded-xl flex items-center gap-x-2 text-slate-600 hover:text-slate-900 transition-colors"
                                >
                                    <i class="fa-solid fa-sync fa-sm"></i>
                                    <span>Refresh</span>
                                </button>
                            </div>

                            <div class="divide-y">
                                <For
                                    each=projects
                                    key=|p| p.id.unwrap_or(0)
                                    children=move |project| {
                                        view! {
                                            <div class="project-card px-6 py-5 flex items-start gap-x-4 hover:bg-slate-50/60">
                                                <div class="w-9 h-9 mt-0.5 flex-shrink-0 bg-blue-100 text-blue-600 rounded-2xl flex items-center justify-center">
                                                    <i class="fa-solid fa-folder fa-lg"></i>
                                                </div>
                                                <div class="flex-1 min-w-0">
                                                    <div class="flex items-center gap-x-3">
                                                        <div class="font-semibold text-lg tracking-tight text-slate-900">
                                                            {project.name.clone().unwrap_or_else(|| "Unnamed".to_string())}
                                                        </div>
                                                        {project.projectKey.as_ref().map(|key| view! {
                                                            <div class="font-mono text-xs px-2.5 py-0.5 bg-slate-100 text-slate-500 rounded-lg">{key.clone()}</div>
                                                        })}
                                                        {project.status.as_ref().map(|s| view! {
                                                            <div class=move || format!("text-[10px] px-2 py-px rounded font-medium tracking-wider {}",
                                                                if s.to_uppercase() == "ACTIVE" { "bg-emerald-100 text-emerald-600" } else { "bg-slate-100 text-slate-500" }
                                                            )>
                                                                {s.clone()}
                                                            </div>
                                                        })}
                                                    </div>
                                                    
                                                    {project.description.as_ref().map(|desc| view! {
                                                        <div class="text-sm text-slate-600 mt-1.5 line-clamp-2 pr-8">{desc.clone()}</div>
                                                    })}
                                                    
                                                    <div class="mt-3 flex items-center gap-x-4 text-xs text-slate-400">
                                                        <div class="font-mono">ID: {project.id.unwrap_or(0)}</div>
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                    }
                                />
                            </div>
                        </div>
                    }.into_view()
                } else if has_token() && !is_loading.get() {
                    view! {
                        <div class="text-center py-10 px-6 bg-white border border-dashed border-slate-200 rounded-3xl">
                            <i class="fa-solid fa-folder-open text-4xl text-slate-300 mb-4"></i>
                            <p class="text-slate-500">Click \"Fetch Projects\" to load your Jama projects.</p>
                        </div>
                    }.into_view()
                } else {
                    view! { <div></div> }.into_view()
                }}
            </div>

            // Footer note
            <div class="max-w-5xl mx-auto px-6 pb-12 text-center">
                <p class="text-[10px] text-slate-400">
                    This is a client-side Leptos demo. For production use, consider a backend proxy to avoid CORS issues.
                </p>
            </div>
        </div>
    }
}

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
