use leptos::prelude::*;

struct Role {
    title: &'static str,
    org: &'static str,
    date: &'static str,
    desc: &'static str,
}

#[component]
pub fn Experience() -> impl IntoView {
    let history = vec![
        
        Role {
            title: "Core Team Member",
            org: "Saturnalia (Techno-Cultural Fest)",
            date: "Sep 2025 - Nov 2025",
            desc: "Streamlined sponsor communication for North India's largest fest. Managed Marketing for 100+ events and optimized proposal decks.",
        },
        Role {
            title: "Frosh Mentor",
            org: "Thapar Institute",
            date: "Jun 2025 - Aug 2025",
            desc: "Mentored 4000+ students and orchestrated a 30-day event series, personally spearheading 77% of total sponsorship acquisition.",
        },
        Role {
            title: "Research Intern",
            org: "ELC (Thapar Institute)",
            date: "May 2025 - July 2025",
            desc: "Conducted blockchain-based ransomware detection study. Developed an ensemble of 3 AI models and implemented immutable infrastructure logging.",
        },
    ];

    view! {
        <section id="experience" class="max-w-4xl mx-auto px-6 py-24">
            <div class="flex items-center gap-4 mb-16">
                <div class="h-px bg-zinc-800 flex-1"></div>
                <h2 class="text-2xl font-bold text-orange-500 uppercase tracking-widest">"// Execution_Log"</h2>
                <div class="h-px bg-zinc-800 flex-1"></div>
            </div>

            <div class="relative border-l border-zinc-800 ml-3 md:ml-6 space-y-12">
                {history.into_iter().map(|role| view! {
                    <div class="relative pl-8 md:pl-12 group">
                        // Timeline Dot
                        <div class="absolute -left-[5px] top-2 w-2.5 h-2.5 rounded-full bg-zinc-800 border border-zinc-950 group-hover:bg-orange-500 group-hover:shadow-[0_0_10px_rgba(249,115,22,0.6)] transition-all duration-300"></div>
                        
                        // Content Card
                        <div class="flex flex-col sm:flex-row sm:justify-between sm:items-baseline mb-2">
                            <h3 class="text-xl font-bold text-white group-hover:text-orange-400 transition-colors">
                                {role.title}
                            </h3>
                            <span class="font-mono text-xs text-zinc-500 border border-zinc-800 px-2 py-1 rounded mt-2 sm:mt-0">
                                {role.date}
                            </span>
                        </div>
                        
                        <div class="text-sm font-mono text-orange-500/80 mb-3">
                            "@" {role.org}
                        </div>
                        
                        <p class="text-zinc-400 leading-relaxed text-sm max-w-2xl">
                            {role.desc}
                        </p>
                    </div>
                }).collect::<Vec<_>>()}
            </div>
        </section>
    }
}