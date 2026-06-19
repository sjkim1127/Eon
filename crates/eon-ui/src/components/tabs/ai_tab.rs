use dioxus::prelude::*;
use std::rc::Rc;

#[derive(Clone, PartialEq)]
enum Role {
    User,
    Assistant,
}

#[derive(Clone, PartialEq)]
struct Message {
    id: usize,
    role: Role,
    content: String,
}

#[component]
pub fn AiTab() -> Element {
    let mut messages = use_signal(|| vec![
        Message {
            id: 0,
            role: Role::Assistant,
            content: "안녕하세요! Eon AI입니다. 사주, 베딕, 운세 분석에 대해 무엇이든 물어보세요.\n\n예: '내 사주의 강점은 뭐야?', '올해 연애운이 어때?'".to_string(),
        }
    ]);
    let mut input_text = use_signal(String::new);
    let mut is_loading = use_signal(|| false);

    let handle_submit = move |_| {
        let text = input_text.read().trim().to_string();
        if text.is_empty() { return; }

        let current_id = messages.read().len();
        
        messages.write().push(Message {
            id: current_id,
            role: Role::User,
            content: text,
        });
        
        input_text.write().clear();
        is_loading.write().clone_from(&true);

        // Mock AI response
        spawn(async move {
            gloo_timers::future::TimeoutFuture::new(1500).await;
            
            let id = messages.read().len();
            messages.write().push(Message {
                id,
                role: Role::Assistant,
                content: "분석을 위해 잠시만 기다려주세요... (이것은 Dioxus로 구현된 AI 챗봇 시뮬레이션입니다.)".to_string(),
            });
            is_loading.write().clone_from(&false);
        });
    };

    rsx! {
        div { class: "flex flex-col h-[calc(100vh-6rem)] max-h-[800px] w-full bg-slate-900 border border-slate-800 rounded-2xl overflow-hidden shadow-2xl",
            // Header
            div { class: "h-14 border-b border-slate-800 bg-slate-900/50 backdrop-blur flex items-center px-6 shrink-0",
                div { class: "flex items-center gap-3",
                    span { class: "text-2xl", "✨" }
                    div {
                        h3 { class: "font-semibold text-slate-200", "Eon AI Assistant" }
                        p { class: "text-xs text-slate-400", "Llama 3 (Groq) 연동 대기중" }
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
                                div { class: "w-2 h-2 rounded-full bg-slate-500 animate-bounce" }
                                div { class: "w-2 h-2 rounded-full bg-slate-500 animate-bounce delay-75" }
                                div { class: "w-2 h-2 rounded-full bg-slate-500 animate-bounce delay-150" }
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
                        class: "w-full min-h-[56px] max-h-48 bg-slate-800/80 border border-slate-700 text-slate-200 rounded-2xl px-5 py-4 focus:outline-none focus:ring-2 focus:ring-violet-500/50 focus:border-violet-500 resize-none placeholder-slate-500 shadow-inner",
                        placeholder: "분석하고 싶은 내용을 입력하세요...",
                        rows: "1",
                        value: "{input_text}",
                        oninput: move |evt| input_text.write().clone_from(&evt.value()),
                        onkeydown: move |evt| {
                            if evt.key() == dioxus::html::input_data::keyboard_types::Key::Enter && !evt.modifiers().contains(dioxus::html::input_data::keyboard_types::Modifiers::SHIFT) {
                                evt.prevent_default();
                                // trigger submit conceptually
                            }
                        }
                    }
                    
                    button {
                        r#type: "submit",
                        disabled: *is_loading.read() || input_text.read().trim().is_empty(),
                        class: "shrink-0 w-14 h-14 rounded-2xl bg-violet-600 hover:bg-violet-500 text-white flex items-center justify-center transition-all disabled:opacity-50 disabled:hover:bg-violet-600",
                        span { class: "text-xl", "↑" }
                    }
                }
                div { class: "text-center mt-2",
                    span { class: "text-[10px] text-slate-500", "AI는 실수를 할 수 있습니다. 중요한 결정은 전문가와 상의하세요." }
                }
            }
        }
    }
}

#[component]
fn ChatMessage(msg: Message) -> Element {
    let is_user = msg.role == Role::User;
    
    let wrapper_class = if is_user { "flex justify-end" } else { "flex justify-start" };
    let bubble_class = if is_user {
        "bg-violet-600 text-slate-100 rounded-2xl rounded-tr-sm border border-violet-500 shadow-sm"
    } else {
        "bg-slate-800/80 text-slate-300 rounded-2xl rounded-tl-sm border border-slate-700/50 shadow-sm"
    };

    rsx! {
        div { class: "{wrapper_class} group animate-in fade-in slide-in-from-bottom-2",
            div { class: "max-w-[85%] md:max-w-[75%] {bubble_class} px-5 py-4 whitespace-pre-wrap leading-relaxed",
                "{msg.content}"
            }
        }
    }
}
