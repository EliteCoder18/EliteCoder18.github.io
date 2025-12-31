use leptos::prelude::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="fixed top-0 w-full z-50 bg-zinc-950/90 backdrop-blur-md border-b border-zinc-800">
            <div class="max-w-7xl mx-auto px-6 py-4 flex justify-between items-center">
                <div class="font-mono font-bold text-xl text-white">
                    "./rm"<span class="text-orange-500 blink">"_"</span>
                </div>
                
                <div class="hidden md:flex space-x-8 font-mono text-sm text-zinc-400">
                    <a href="#home" class="hover:text-orange-500 transition">"~/home"</a>
                    <a href="#projects" class="hover:text-orange-500 transition">"~/projects"</a>
                    <a href="#contact" class="text-orange-500 border border-orange-500 px-4 py-1 hover:bg-orange-500 hover:text-black transition">"contact.sh"</a>
                </div>
            </div>
        </nav>
    }
}