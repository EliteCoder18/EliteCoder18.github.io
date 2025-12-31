use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    let current_year = 2026; // You can make this dynamic if you want

    view! {
        <footer class="w-full border-t border-zinc-800 bg-black/80 backdrop-blur-md mt-auto relative z-50 font-mono text-sm">
            
            // --- TOP BAR: STATUS TICKER ---
            <div class="w-full bg-zinc-900/50 border-b border-zinc-800 py-1 px-4 flex justify-between items-center text-[10px] uppercase tracking-widest text-zinc-500">
                <div class="flex items-center gap-4">
                    <span class="flex items-center gap-2">
                        <span class="w-2 h-2 bg-green-500 rounded-full animate-pulse"></span>
                        "SYSTEM_ONLINE"
                    </span>
                    <span class="hidden md:inline">"LATENCY: 12ms"</span>
                    <span class="hidden md:inline">"MEM_USAGE: LOW"</span>
                </div>
                <div>
                    "RUST_VERSION: 1.92.0"
                </div>
            </div>

            // --- MAIN CONTENT ---
            <div class="max-w-7xl mx-auto px-6 py-12 grid grid-cols-1 md:grid-cols-3 gap-8 text-zinc-400">
                
                // COLUMN 1: IDENTITY
                <div class="flex flex-col gap-4">
                    <h3 class="text-white font-bold text-lg flex items-center gap-2">
                        <span class="text-orange-500">">"</span> "Rishit Modi"
                    </h3>
                    <p class="text-xs leading-relaxed max-w-xs">
                        "Full-stack Systems Engineer specializing in Rust, WebAssembly, and Blockchain architecture. Building the digital bedrock."
                    </p>
                </div>

                // COLUMN 2: UPLINKS (Socials)
                <div class="flex flex-col gap-4">
                    <h3 class="text-xs font-bold text-zinc-500 uppercase tracking-widest">"// Establish_Uplink"</h3>
                    <div class="flex gap-4">
                        // GITHUB
                        <a href="https://github.com/EliteCoder18" target="_blank" class="text-zinc-400 hover:text-white hover:scale-110 transition-transform">
                            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.403 5.403 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4"/>
                            <path d="M9 18c-4.51 2-5-2-7-2"/> 
                            </svg>
                        </a>
                        
                        // LINKEDIN
                        <a href="https://linkedin.com/in/rishit-modi" target="_blank" class="text-zinc-400 hover:text-blue-400 hover:scale-110 transition-transform">
                            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M16 8a6 6 0 0 1 6 6v7h-4v-7a2 2 0 0 0-2-2 2 2 0 0 0-2 2v7h-4v-7a6 6 0 0 1 6-6z"/><rect width="4" height="12" x="2" y="9"/><circle cx="4" cy="4" r="2"/></svg>
                        </a>

                        // EMAIL
                        <a href="rishitmodi338@mail.com" class="text-zinc-400 hover:text-orange-500 hover:scale-110 transition-transform">
                            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="20" height="16" x="2" y="4" rx="2"/><path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"/></svg>
                        </a>
                    </div>
                </div>

                // COLUMN 3: TECH STACK BADGE
                <div class="flex flex-col gap-4">
                    <h3 class="text-xs font-bold text-zinc-500 uppercase tracking-widest">"// This site is Powered_By"</h3>
                    <div class="flex flex-wrap gap-2">
                         <span class="px-2 py-1 bg-orange-900/20 border border-orange-500/30 text-orange-500 text-[10px] rounded">"RUST"</span>
                         <span class="px-2 py-1 bg-blue-900/20 border border-blue-500/30 text-blue-500 text-[10px] rounded">"LEPTOS"</span>
                         <span class="px-2 py-1 bg-purple-900/20 border border-purple-500/30 text-purple-500 text-[10px] rounded">"WASM"</span>
                         <span class="px-2 py-1 bg-cyan-900/20 border border-cyan-500/30 text-cyan-500 text-[10px] rounded">"TAILWIND"</span>
                    </div>
                </div>
            </div>

            // --- BOTTOM BAR: HASH & COPYRIGHT ---
            <div class="bg-black py-4 border-t border-zinc-900 text-center">
                <p class="text-[10px] text-zinc-600">
                    "BLOCK_HEIGHT: "{current_year}" // HASH: 0x7F2A...9C // "
                    <span class="text-zinc-500">"© RISHIT MODI. ALL NODES RESERVED."</span>
                </p>
            </div>
        </footer>
    }
}