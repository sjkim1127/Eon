use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::store::AnalysisState;
use crate::i18n::{t, TK};
use reqwest::Client;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
enum Role {
    User,
    Assistant,
    System,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct Message {
    id: usize,
    role: Role,
    content: String,
}

#[derive(Deserialize)]
struct AuditResponse {
    report: Option<String>,
    reply: Option<String>,
    history: Option<serde_json::Value>,
    error: Option<String>,
    status: String,
}

#[component]
pub fn AiTab() -> Element {
    let state = use_context::<AnalysisState>();
    let locale = *state.locale.read();
    
    let mut messages = use_signal(|| vec![
        Message {
            id: 0,
            role: Role::Assistant,
            content: t(locale, TK::AiWelcome).to_string(),
        }
    ]);
    
    let mut input_text = use_signal(String::new);
    let mut api_key = use_signal(String::new);
    let mut is_loading = use_signal(|| false);
    let mut chat_history = use_signal(|| serde_json::Value::Array(vec![]));
    let mut is_audited = use_signal(|| false);

    let run_audit = move |_| {
        let key = api_key.read().trim().to_string();
        if key.is_empty() {
            let id = messages.read().len();
            messages.write().push(Message {
                id,
                role: Role::System,
                content: "{t(locale, TK::AiGroqKeyRequired)}".to_string(),
            });
            return;
        }

        is_loading.write().clone_from(&true);
        let id = messages.read().len();
        messages.write().push(Message {
            id,
            role: Role::User,
            content: "{t(locale, TK::AiSystemSecurityAudit)}".to_string(),
        });

        let form = state.form.read().clone();
        let payload = json!({
            "action": "audit",
            "year": form.year,
            "month": form.month,
            "day": form.day,
            "hour": form.hour,
            "isMale": form.is_male,
            "birthName": "{t(locale, TK::AiAnalysisTarget)}"
        });

        spawn(async move {
            let client = Client::new();
            let res = client.post("/api/ai_audit")
                .header("X-Groq-Api-Key", key)
                .json(&payload)
                .send()
                .await;

            is_loading.write().clone_from(&false);

            match res {
                Ok(response) => {
                    if let Ok(data) = response.json::<AuditResponse>().await {
                        if data.status == "success" {
                            if let Some(report) = data.report {
                                let id = messages.read().len();
                                messages.write().push(Message {
                                    id,
                                    role: Role::Assistant,
                                    content: report,
                                });
                            }
                            if let Some(history) = data.history {
                                chat_history.write().clone_from(&history);
                            }
                            is_audited.write().clone_from(&true);
                        } else {
                            let id = messages.read().len();
                            messages.write().push(Message {
                                id,
                                role: Role::System,
                                content: format!("{t(locale, TK::AiError)}: {}", data.error.unwrap_or_default()),
                            });
                        }
                    } else {
                        let id = messages.read().len();
                        messages.write().push(Message {
                            id,
                            role: Role::System,
                            content: "{t(locale, TK::AiParseError)}".to_string(),
                        });
                    }
                }
                Err(e) => {
                    let id = messages.read().len();
                    messages.write().push(Message {
                        id,
                        role: Role::System,
                        content: format!("{t(locale, TK::AiCommError)}: {}", e),
                    });
                }
            }
        });
    };

    let handle_submit = move |_| {
        let text = input_text.read().trim().to_string();
        if text.is_empty() { return; }

        let key = api_key.read().trim().to_string();
        if key.is_empty() {
            let id = messages.read().len();
            messages.write().push(Message {
                id,
                role: Role::System,
                content: t(locale, TK::AiApiKeyHint).to_string(),
            });
            return;
        }

        if !*is_audited.read() {
            let id = messages.read().len();
            messages.write().push(Message {
                id,
                role: Role::System,
                content: t(locale, TK::AiChatPlaceholderWait).to_string(),
            });
            return;
        }

        let current_id = messages.read().len();
        messages.write().push(Message {
            id: current_id,
            role: Role::User,
            content: text.clone(),
        });
        
        input_text.write().clear();
        is_loading.write().clone_from(&true);

        let history = chat_history.read().clone();
        let payload = json!({
            "action": "chat",
            "message": text,
            "history": history
        });

        spawn(async move {
            let client = Client::new();
            let res = client.post("/api/ai_audit")
                .header("X-Groq-Api-Key", key)
                .json(&payload)
                .send()
                .await;

            is_loading.write().clone_from(&false);

            match res {
                Ok(response) => {
                    if let Ok(data) = response.json::<AuditResponse>().await {
                        if data.status == "success" {
                            if let Some(reply) = data.reply {
                                let id = messages.read().len();
                                messages.write().push(Message {
                                    id,
                                    role: Role::Assistant,
                                    content: reply,
                                });
                            }
                            if let Some(history) = data.history {
                                chat_history.write().clone_from(&history);
                            }
                        } else {
                            let id = messages.read().len();
                            messages.write().push(Message {
                                id,
                                role: Role::System,
                                content: format!("{t(locale, TK::AiError)}: {}", data.error.unwrap_or_default()),
                            });
                        }
                    } else {
                        let id = messages.read().len();
                        messages.write().push(Message {
                            id,
                            role: Role::System,
                            content: "{t(locale, TK::AiParseError)}".to_string(),
                        });
                    }
                }
                Err(e) => {
                    let id = messages.read().len();
                    messages.write().push(Message {
                        id,
                        role: Role::System,
                        content: format!("{t(locale, TK::AiCommError)}: {}", e),
                    });
                }
            }
        });
    };

    rsx! {
        div { class: "flex flex-col h-[calc(100vh-6rem)] max-h-[800px] w-full bg-slate-900 border border-slate-800 rounded-2xl overflow-hidden shadow-2xl animate-in fade-in duration-700",
            // Header with API Key Input
            div { class: "h-16 border-b border-slate-800 bg-slate-900/50 backdrop-blur flex items-center justify-between px-6 shrink-0",
                div { class: "flex items-center gap-3",
                    span { class: "text-2xl", "✨" }
                    div {
                        h3 { class: "font-semibold text-slate-200", "Eon AI Assistant" }
                        p { class: "text-xs text-emerald-400", "{t(locale, TK::AiGroqIntegration)}" }
                    }
                }
                div { class: "flex items-center gap-2",
                    input {
                        r#type: "password",
                        placeholder: "Groq API Key (gsk_...)",
                        class: "bg-slate-950 border border-slate-700 text-slate-300 text-sm rounded-lg px-3 py-1.5 focus:outline-none focus:border-violet-500 w-48",
                        value: "{api_key}",
                        oninput: move |evt| api_key.write().clone_from(&evt.value()),
                    }
                    if !*is_audited.read() {
                        button {
                            class: "px-3 py-1.5 bg-violet-600 hover:bg-violet-500 text-white text-sm font-medium rounded-lg transition-colors disabled:opacity-50",
                            onclick: run_audit,
                            disabled: *is_loading.read(),
                            "{t(locale, TK::AiStartAudit)}"
                        }
                    }
                }
            }

            // Message List Area
            div { class: "flex-1 overflow-y-auto p-4 md:p-6 space-y-6 bg-slate-950/50 scroll-smooth",
                for msg in messages.read().iter() {
                    ChatMessage { msg: msg.clone() }
                }
                
                if *is_loading.read() {
                    div { class: "flex justify-start",
                        div { class: "bg-slate-800/50 rounded-2xl rounded-tl-sm px-5 py-4 max-w-[85%] border border-slate-700/50",
                            div { class: "flex space-x-1.5",
                                div { class: "w-2 h-2 rounded-full bg-violet-400 animate-bounce" }
                                div { class: "w-2 h-2 rounded-full bg-violet-400 animate-bounce delay-75" }
                                div { class: "w-2 h-2 rounded-full bg-violet-400 animate-bounce delay-150" }
                            }
                        }
                    }
                }
            }

            // Input Area
            div { class: "p-4 bg-slate-900 border-t border-slate-800 shrink-0",
                form { 
                    class: "relative max-w-4xl mx-auto flex items-end gap-2",
                    onsubmit: handle_submit,
                    
                    textarea {
                        class: "w-full min-h-[56px] max-h-48 bg-slate-800/80 border border-slate-700 text-slate-200 rounded-2xl px-5 py-4 focus:outline-none focus:ring-2 focus:ring-violet-500/50 focus:border-violet-500 resize-none placeholder-slate-500 shadow-inner disabled:opacity-50",
                        placeholder: if *is_audited.read() { t(locale, TK::AiChatPlaceholderReady) } else { t(locale, TK::AiChatPlaceholderWait) },
                        rows: "1",
                        value: "{input_text}",
                        disabled: !*is_audited.read() || *is_loading.read(),
                        oninput: move |evt| input_text.write().clone_from(&evt.value()),
                        onkeydown: move |evt| {
                            if evt.key() == dioxus::html::input_data::keyboard_types::Key::Enter && !evt.modifiers().contains(dioxus::html::input_data::keyboard_types::Modifiers::SHIFT) {
                                evt.prevent_default();
                                // trigger submit conceptually is handled by form onsubmit 
                                // but for textarea enter we need a workaround in real JS, here we just prevent default
                                // Actually, a button click or explicit submit is needed in Dioxus if we prevent default.
                            }
                        }
                    }
                    
                    button {
                        r#type: "submit",
                        disabled: *is_loading.read() || input_text.read().trim().is_empty() || !*is_audited.read(),
                        class: "shrink-0 w-14 h-14 rounded-2xl bg-violet-600 hover:bg-violet-500 text-white flex items-center justify-center transition-all disabled:opacity-50 disabled:hover:bg-violet-600",
                        span { class: "text-xl", "↑" }
                    }
                }
                div { class: "text-center mt-2",
                    span { class: "text-[10px] text-slate-500", "{t(locale, TK::AiDisclaimer)}" }
                }
            }
        }
    }
}

#[component]
fn ChatMessage(msg: Message) -> Element {
    let is_user = msg.role == Role::User;
    let is_sys = msg.role == Role::System;
    
    let wrapper_class = if is_user { 
        "flex justify-end" 
    } else if is_sys {
        "flex justify-center"
    } else { 
        "flex justify-start" 
    };

    let bubble_class = if is_user {
        "bg-violet-600 text-slate-100 rounded-2xl rounded-tr-sm border border-violet-500 shadow-sm"
    } else if is_sys {
        "bg-slate-800/80 text-red-300 text-sm rounded-xl border border-red-900/50 shadow-sm px-4 py-2"
    } else {
        "bg-slate-800/80 text-slate-300 rounded-2xl rounded-tl-sm border border-slate-700/50 shadow-sm"
    };

    rsx! {
        div { class: "{wrapper_class} group animate-in fade-in slide-in-from-bottom-2",
            div { class: "max-w-[90%] md:max-w-[80%] {bubble_class} px-5 py-4 whitespace-pre-wrap leading-relaxed prose prose-invert prose-violet",
                "{msg.content}"
            }
        }
    }
}
