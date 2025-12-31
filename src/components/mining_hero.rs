use leptos::prelude::*;
use leptos::leptos_dom::helpers::set_timeout;
use std::time::Duration;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
struct CodeToken {
    text: &'static str,
    class: &'static str,
}

#[component]
pub fn MiningHero() -> impl IntoView {
    // 1. DEFINE TOKENS
    let tokens = vec![
        CodeToken { text: "struct", class: "text-purple-400" },
        CodeToken { text: " ", class: "text-zinc-300" },
        CodeToken { text: "Developer", class: "text-yellow-300" },
        CodeToken { text: " {", class: "text-zinc-300" },
        CodeToken { text: "\n  ", class: "text-zinc-300" },
        
        CodeToken { text: "name", class: "text-blue-400" },
        CodeToken { text: ": ", class: "text-zinc-300" },
        CodeToken { text: "\"Rishit\"", class: "text-green-400" },
        CodeToken { text: ",", class: "text-zinc-300" },
        CodeToken { text: "\n  ", class: "text-zinc-300" },

        CodeToken { text: "role", class: "text-blue-400" },
        CodeToken { text: ": ", class: "text-zinc-300" },
        CodeToken { text: "\"Full Stack Engineer\"", class: "text-green-400" },
        CodeToken { text: ",", class: "text-zinc-300" },
        CodeToken { text: "\n  ", class: "text-zinc-300" },

        CodeToken { text: "stack", class: "text-blue-400" },
        CodeToken { text: ": ", class: "text-zinc-300" },
        CodeToken { text: "vec!", class: "text-purple-400" },
        CodeToken { text: "[", class: "text-zinc-300" },
        CodeToken { text: "\"Rust\"", class: "text-green-400" },
        CodeToken { text: ", ", class: "text-zinc-300" },
        CodeToken { text: "\"Wasm\"", class: "text-green-400" },
        CodeToken { text: "],", class: "text-zinc-300" },
        CodeToken { text: "\n", class: "text-zinc-300" },
        
        CodeToken { text: "}", class: "text-zinc-300" },
        CodeToken { text: "\n", class: "text-zinc-300" },
        CodeToken { text: "fn", class: "text-purple-400" },
        CodeToken { text: " ", class: "text-zinc-300" },
        CodeToken { text: "main", class: "text-yellow-300" },
        CodeToken { text: "() {", class: "text-zinc-300" },
        CodeToken { text: "\n  ", class: "text-zinc-300" },
        CodeToken { text: "contact_me();", class: "text-zinc-100" },
        CodeToken { text: "\n", class: "text-zinc-300" },
        CodeToken { text: "}", class: "text-zinc-300" },
    ];

    let total_chars: usize = tokens.iter().map(|t| t.text.len()).sum();
    let (char_count, set_char_count) = signal(0);

    // 2. RECURSIVE TYPING LOGIC
    Effect::new(move |_| {
        let counter = Rc::new(RefCell::new(0));
        let max_chars = total_chars;
        
        let loop_fn: Rc<RefCell<Option<Box<dyn Fn()>>>> = Rc::new(RefCell::new(None));
        let loop_fn_clone = loop_fn.clone();

        *loop_fn_clone.borrow_mut() = Some(Box::new(move || {
            let current = *counter.borrow();
            
            if current < max_chars {
                set_char_count.set(current + 1);
                *counter.borrow_mut() += 1;
                
                let next = loop_fn.clone();
                set_timeout(move || {
                    if let Some(f) = next.borrow().as_ref() {
                        f();
                    }
                }, Duration::from_millis(30));
            }
        }));
        
        if let Some(f) = loop_fn_clone.borrow().as_ref() {
            f();
        }
    });

    view! {
        <section class="w-full max-w-5xl mx-auto px-4 pt-32 pb-12 flex flex-col items-center justify-center min-h-[60vh]">
            
            // --- ADDED: Custom Blink Animation ---
            <style>
                "@keyframes cursor-blink {
                    0%, 100% { opacity: 1; }
                    50% { opacity: 0; }
                }
                .animate-cursor-blink {
                    animation: cursor-blink 1s step-end infinite;
                }"
            </style>

            <div class="text-center mb-10 z-10">
            // 1. NAME (The Main Headline)
            <h1 class="text-5xl md:text-7xl font-bold text-zinc-100 tracking-tight mb-4">
                "Hello, I'm " 
                <span class="text-transparent bg-clip-text bg-gradient-to-r from-orange-400 to-red-500">
                    "Rishit"
                </span>
            </h1>

            // 2. TAGLINE (The "Logic. Systems. Magic." goes here)
            <h3 class="text-2xl md:text-3xl font-bold text-zinc-300 mt-4">
                "Logic. Systems. "
                <span class="text-transparent bg-clip-text bg-gradient-to-r from-amber-300 to-orange-500">
                    "Magic."
                </span>
            </h3>
            
            // 3. SUBTEXT (Optional: Keep the 'Rust' line small below if you want)
            <p class="text-sm text-zinc-500 mt-3 font-mono tracking-wide">
    "It all started with a Hello World."
</p>
        </div>

            <div class="w-full max-w-3xl relative group z-10">
                <div class="absolute -inset-1 bg-gradient-to-r from-blue-500/20 to-purple-500/20 rounded-xl blur-xl opacity-30 group-hover:opacity-50 transition duration-1000"></div>
                
                <div class="relative bg-[#1e1e1e] rounded-xl shadow-2xl overflow-hidden border border-zinc-800">
                    
                    <div class="bg-[#252526] px-4 py-3 flex items-center justify-between border-b border-[#333]">
                        <div class="flex items-center gap-2">
                            <div class="w-3 h-3 rounded-full bg-[#ff5f56]"></div>
                            <div class="w-3 h-3 rounded-full bg-[#ffbd2e]"></div>
                            <div class="w-3 h-3 rounded-full bg-[#27c93f]"></div>
                        </div>
                        <div class="text-xs text-zinc-500 font-mono flex items-center gap-2">
                             <span class="text-blue-400">"⚡"</span> "main.rs"
                        </div>
                        <div class="w-10"></div>
                    </div>

                    // Code Content
                    <div class="p-6 md:p-8 font-mono text-sm md:text-base leading-relaxed overflow-x-auto bg-[#1e1e1e] min-h-[300px] whitespace-pre-wrap">
                        {
                            let mut current_idx = 0;
                            tokens.into_iter().map(|token| {
                                let token_len = token.text.len();
                                let start = current_idx;
                                let end = start + token_len;
                                current_idx = end;

                                move || {
                                    let count = char_count.get();
                                    if count <= start {
                                        view! { <span class={token.class}></span> }.into_any()
                                    } else if count >= end {
                                        view! { <span class={token.class}>{token.text}</span> }.into_any()
                                    } else {
                                        let slice_len = count - start;
                                        let slice = &token.text[0..slice_len];
                                        view! { 
                                            <span class={token.class}>
                                                {slice}
                                                <span class="inline-block w-2 h-4 bg-orange-500 animate-cursor-blink align-middle ml-0.5"></span>
                                            </span> 
                                        }.into_any()
                                    }
                                }
                            }).collect::<Vec<_>>()
                        }

                        {move || {
                            if char_count.get() >= total_chars {
                                view! { <span class="inline-block w-2 h-4 bg-orange-500 animate-cursor-blink align-middle ml-0.5"></span> }.into_any()
                            } else {
                                view! { <span class="hidden"></span> }.into_any()
                            }
                        }}
                    </div>

                    <div class="bg-[#007acc] text-white px-3 py-1 text-xs flex justify-between items-center font-sans">
                        <div class="flex gap-4">
                            <span>"master*"</span>
                            <span>"0 errors"</span>
                        </div>
                        <div class="flex gap-4">
                            <span>"Ln 15, Col 1"</span>
                            <span>"Rust"</span>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}