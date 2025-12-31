use leptos::prelude::*;
use crate::models::Project;
use crate::components::navbar::Navbar;
use crate::components::project_card::ProjectCard;
use crate::components::contact_form::ContactForm;
use crate::components::mining_hero::MiningHero;
use crate::components::footer::Footer;
use crate::components::block_pit::BlockPit; // Import the new component

#[component]
pub fn App() -> impl IntoView {
    let projects = vec![
        Project {
            id: 1,
            title: "Rust_Physics_Engine".into(),
            description: "Simulating rigid body dynamics using WASM. High-performance collision detection implementation.".into(),
            tech_stack: vec!["Rust".into(), "WebAssembly".into(), "Linear Algebra".into()],
            link: "#".into(),
        },
        Project {
            id: 2,
            title: "DeFi_Dashboard".into(),
            description: "Real-time visualization of blockchain transactions. Connects to Ethereum nodes via WebSocket.".into(),
            tech_stack: vec!["Leptos".into(), "Ethers.rs".into(), "Tailwind".into()],
            link: "#".into(),
        },
    ];

    view! {
        <div class="bg-mine-depths min-h-screen text-zinc-200 font-mono selection:bg-orange-500 selection:text-black">
            <Navbar />
            
            // --- HERO SECTION ---
            <section id="home" class="relative pt-24 pb-20 px-6 min-h-[90vh] flex flex-col items-center justify-center">
                // Background Grid
                <div class="absolute inset-0 bg-grid-pattern opacity-20 pointer-events-none"></div>

                // Headline
                <div class="text-center relative z-10 mb-8">
                    <h1 class="text-4xl md:text-6xl font-black tracking-tighter text-white mb-2">
                        "BUILDING "<span class="text-transparent bg-clip-text bg-gradient-to-r from-orange-500 to-red-600">"BLOCKS"</span>
                    </h1>
                    <p class="text-zinc-500 text-lg">"High-Performance Systems & WebAssembly"</p>
                </div>

                <MiningHero />
            </section>

            // --- PROJECTS ---
            <section id="projects" class="max-w-6xl mx-auto px-6 py-24">
                <div class="flex items-center gap-4 mb-12">
                    <div class="h-px bg-zinc-800 flex-1"></div>
                    <h2 class="text-2xl font-bold text-orange-500 uppercase tracking-widest">"// Mined_Blocks"</h2>
                    <div class="h-px bg-zinc-800 flex-1"></div>
                </div>
                
                <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                    // If you still get errors here, try removing the `move ||` before projects.clone()
                    // But usually, explicit imports fix it.
                    <For
                        each=move || projects.clone()
                        key=|p| p.id
                        children=|p| view! { <ProjectCard project=p /> }
                    />
                </div>
            </section>

            // --- CONTACT ---
            <section id="contact" class="py-24 px-6 relative">
                 <div class="absolute inset-0 bg-zinc-900 skew-y-3 transform origin-bottom-left -z-10 border-t border-orange-500/20"></div>
                 <div class="max-w-xl mx-auto text-center mb-10">
                    <h2 class="text-2xl font-bold text-white uppercase tracking-widest">"// Broadcast_Tx"</h2>
                 </div>
                 <ContactForm />
            </section>

            // --- NEW BLOCK PIT ---
            <BlockPit />

            <Footer />
        </div>
    }
}