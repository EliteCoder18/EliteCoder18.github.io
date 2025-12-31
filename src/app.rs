use leptos::prelude::*;

use crate::components::console_guard::ConsoleGuard;
use crate::components::navbar::Navbar;

use crate::components::contact_form::Contact;
use crate::components::mining_hero::MiningHero;
use crate::components::footer::Footer;
use crate::components::block_pit::BlockPit; 
use crate::components::projects::Projects;
use crate::components::experience::Experience;
#[component]
pub fn App() -> impl IntoView {

    view! {
        <ConsoleGuard />
        <div class="bg-mine-depths min-h-screen text-zinc-200 font-mono selection:bg-orange-500 selection:text-black">
            <Navbar />
            
            // --- HERO SECTION ---
            <section id="home" class="relative pt-24 pb-20 px-6 min-h-[90vh] flex flex-col items-center justify-center">
                // Background Grid
                <div class="absolute inset-0 bg-grid-pattern opacity-20 pointer-events-none"></div>

                // Headline
                <div class="text-center relative z-10 mb-2"> 
                 <p class="text-sm font-mono text-orange-500 tracking-widest uppercase">
                        "// SYSTEM_ARCHITECTURE_V1" 
                </p>
                </div>

                <MiningHero />
                
            </section>

            // --- PROJECTS ---
            <Projects />
            // experience
            <Experience/>

            // --- CONTACT ---
            <section id="contact" class="py-24 px-6 relative">
                 <div class="absolute inset-0 bg-zinc-900 skew-y-3 transform origin-bottom-left -z-10 border-t border-orange-500/20"></div>
                 <div class="max-w-xl mx-auto text-center mb-10">
                    <h2 class="text-2xl font-bold text-white uppercase tracking-widest">"// Broadcast_Tx"</h2>
                 </div>
                 <Contact />
            </section>

            // --- NEW BLOCK PIT ---
            <BlockPit />

            <Footer />
        </div>
    }
}