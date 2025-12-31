use leptos::prelude::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="fixed top-0 left-0 w-full z-50 bg-zinc-950/90 backdrop-blur-md border-b border-zinc-800">
            <div class="max-w-7xl mx-auto px-6 h-20 flex justify-between items-center">
                
                // 1. LOGO: ./rm_
                <a href="#home" class="font-mono font-bold text-xl text-white hover:text-orange-500 transition-colors">
                    "./rm"<span class="text-orange-500 animate-pulse">"_"</span>
                </a>
                
                // 2. TERMINAL LINKS
                <div class="hidden md:flex items-center space-x-8 font-mono text-sm text-zinc-400">
                    <a href="#home" class="hover:text-white transition-colors">"~/home"</a>
                    <a href="#projects" class="hover:text-white transition-colors">"~/projects"</a>
                    
                    // 3. RESUME (Themed as 'cat resume.pdf')
                    <a 
                        href="/resume.pdf" 
                        target="_blank" 
                        class="hover:text-white transition-colors"
                    >
                        "cat resume.pdf"
                    </a>

                    // 4. CONTACT BUTTON (Themed as script execution)
                    <a 
                        href="#contact" 
                        class="text-orange-500 border border-orange-500 px-4 py-2 rounded hover:bg-orange-500 hover:text-black transition-all duration-300"
                    >
                        "./contact.sh"
                    </a>
                </div>
            </div>
        </nav>
    }
}