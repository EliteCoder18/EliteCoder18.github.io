use leptos::prelude::*;
use leptos::leptos_dom::helpers::set_timeout;
use std::time::Duration;
use std::rc::Rc;
use std::cell::RefCell;

#[component]
pub fn MiningHero() -> impl IntoView {
    // Controls how many lines are currently visible
    let (visible_lines, set_visible_lines) = signal(0);

    // Animation Effect
    Effect::new(move |_| {
        // We use Rc<RefCell> to allow mutation inside the closure
        let current_line = Rc::new(RefCell::new(0));
        
        // Setup a recursive closure holder
        let f: Rc<RefCell<Option<Box<dyn Fn()>>>> = Rc::new(RefCell::new(None));
        let g = f.clone();

        *g.borrow_mut() = Some(Box::new(move || {
            let c = *current_line.borrow();
            
            // We go up to 8 lines
            if c <= 8 {
                set_visible_lines.set(c);
                *current_line.borrow_mut() += 1;

                let recursive_f = f.clone();
                set_timeout(move || {
                    if let Some(callback) = recursive_f.borrow().as_ref() {
                        callback();
                    }
                }, Duration::from_millis(150)); // Typing speed
            }
        }));

        // Start the animation
        if let Some(callback) = g.borrow().as_ref() {
            callback();
        }
    });

    view! {
        <section class="w-full max-w-5xl mx-auto px-4 pt-32 pb-12 flex flex-col items-center justify-center min-h-[60vh]">
            
            // GREETING
            <div class="text-center mb-10 z-10">
                <h1 class="text-5xl md:text-7xl font-bold text-zinc-100 tracking-tight mb-4">
                    "Hello, I'm " <span class="text-transparent bg-clip-text bg-gradient-to-r from-orange-400 to-red-500">"Rishit"</span>
                </h1>
                <p class="text-xl text-zinc-400 max-w-2xl mx-auto">
                    "I build high-performance web systems with Rust & Wasm."
                </p>
            </div>

            // EDITOR
            <div class="w-full max-w-3xl relative group z-10">
                <div class="absolute -inset-1 bg-gradient-to-r from-blue-500/20 to-purple-500/20 rounded-xl blur-xl opacity-30 group-hover:opacity-50 transition duration-1000"></div>
                
                <div class="relative bg-[#1e1e1e] rounded-xl shadow-2xl overflow-hidden border border-zinc-800">
                    
                    // TITLE BAR
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

                    // EDITOR CONTENT
                    <div class="p-6 md:p-8 font-mono text-sm md:text-base leading-relaxed overflow-x-auto bg-[#1e1e1e] min-h-[300px]">
                        
                        // Line 1
                        <div class={move || if visible_lines.get() >= 1 { "flex" } else { "hidden" }}>
                            <span class="text-zinc-600 select-none w-8 text-right mr-4">"1"</span>
                            <div>
                                <span class="text-pink-400">"struct"</span> <span class="text-yellow-300">" Developer"</span> " {" 
                                {move || if visible_lines.get() == 1 { view!{ <span class="w-2 h-5 bg-orange-500 ml-1 animate-pulse inline-block align-middle"></span> }.into_any() } else { view!{ <span></span> }.into_any() }}
                            </div>
                        </div>

                        // Line 2
                        <div class={move || if visible_lines.get() >= 2 { "flex" } else { "hidden" }}>
                            <span class="text-zinc-600 select-none w-8 text-right mr-4">"2"</span>
                            <div class="pl-4">
                                "name: "<span class="text-[#ce9178]">"\"Rishit\""</span>"," 
                                {move || if visible_lines.get() == 2 { view!{ <span class="w-2 h-5 bg-orange-500 ml-1 animate-pulse inline-block align-middle"></span> }.into_any() } else { view!{ <span></span> }.into_any() }}
                            </div>
                        </div>

                        // Line 3
                        <div class={move || if visible_lines.get() >= 3 { "flex" } else { "hidden" }}>
                            <span class="text-zinc-600 select-none w-8 text-right mr-4">"3"</span>
                            <div class="pl-4">
                                "role: "<span class="text-[#ce9178]">"\"Full Stack Engineer\""</span>"," 
                                {move || if visible_lines.get() == 3 { view!{ <span class="w-2 h-5 bg-orange-500 ml-1 animate-pulse inline-block align-middle"></span> }.into_any() } else { view!{ <span></span> }.into_any() }}
                            </div>
                        </div>

                        // Line 4
                        <div class={move || if visible_lines.get() >= 4 { "flex" } else { "hidden" }}>
                            <span class="text-zinc-600 select-none w-8 text-right mr-4">"4"</span>
                            <div class="pl-4">
                                "stack: "<span class="text-blue-400">"vec!"</span>"["<span class="text-[#ce9178]">"\"Rust\""</span>", "<span class="text-[#ce9178]">"\"Wasm\""</span>"]," 
                                {move || if visible_lines.get() == 4 { view!{ <span class="w-2 h-5 bg-orange-500 ml-1 animate-pulse inline-block align-middle"></span> }.into_any() } else { view!{ <span></span> }.into_any() }}
                            </div>
                        </div>

                        // Line 5
                        <div class={move || if visible_lines.get() >= 5 { "flex" } else { "hidden" }}>
                            <span class="text-zinc-600 select-none w-8 text-right mr-4">"5"</span>
                            <div>"}" {move || if visible_lines.get() == 5 { view!{ <span class="w-2 h-5 bg-orange-500 ml-1 animate-pulse inline-block align-middle"></span> }.into_any() } else { view!{ <span></span> }.into_any() }}</div>
                        </div>

                        // Line 6
                        <div class={move || if visible_lines.get() >= 6 { "flex mt-2" } else { "hidden mt-2" }}>
                            <span class="text-zinc-600 select-none w-8 text-right mr-4">"6"</span>
                            <div class="pl-0 flex items-center">
                                <span class="text-blue-400">"fn"</span> <span class="text-yellow-300 ml-2">"main"</span>"() {" 
                                {move || if visible_lines.get() == 6 { view!{ <span class="w-2 h-5 bg-orange-500 ml-1 animate-pulse inline-block align-middle"></span> }.into_any() } else { view!{ <span></span> }.into_any() }}
                            </div>
                        </div>
                        
                        // Line 7
                        <div class={move || if visible_lines.get() >= 7 { "flex" } else { "hidden" }}>
                            <span class="text-zinc-600 select-none w-8 text-right mr-4">"7"</span>
                            <div class="pl-4 flex items-center">
                                <span class="text-zinc-300">"contact_me();"</span> 
                                {move || if visible_lines.get() == 7 { view!{ <span class="w-2 h-5 bg-orange-500 ml-1 animate-pulse inline-block align-middle"></span> }.into_any() } else { view!{ <span></span> }.into_any() }}
                            </div>
                        </div>

                        // Line 8
                        <div class={move || if visible_lines.get() >= 8 { "flex" } else { "hidden" }}>
                            <span class="text-zinc-600 select-none w-8 text-right mr-4">"8"</span>
                            <div>"}" {move || if visible_lines.get() == 8 { view!{ <span class="w-2 h-5 bg-orange-500 ml-1 animate-pulse inline-block align-middle"></span> }.into_any() } else { view!{ <span></span> }.into_any() }}</div>
                        </div>

                    </div>

                    // STATUS BAR
                    <div class="bg-[#007acc] text-white px-3 py-1 text-xs flex justify-between items-center font-sans">
                        <div class="flex gap-4">
                            <span>"master*"</span>
                            <span>"0 errors"</span>
                        </div>
                        <div class="flex gap-4">
                            <span>"Ln 8, Col 1"</span>
                            <span>"Rust"</span>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}