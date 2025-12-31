use leptos::prelude::*;
use crate::models::Project;

#[component]
pub fn ProjectCard(project: Project) -> impl IntoView {
    view! {
        // Outer "Block" Container
       // Change the border hover color to match the magma feel
        <div class="relative group bg-zinc-900/50 backdrop-blur-sm border border-zinc-700 hover:border-orange-500/50 transition-all duration-300 overflow-hidden">
            
            <div class="h-1 w-full bg-zinc-800 group-hover:bg-orange-500 transition-all duration-500 ease-out"></div>
            
            <div class="p-6 font-mono relative z-10">
                // Block Header: Hash ID styling
                <div class="flex justify-between items-center text-xs text-zinc-500 mb-4 border-b border-zinc-800 pb-2">
                    <span>{format!("BLK_{:03}", project.id)}</span>
                    <span class="font-bold group-hover:text-orange-400 transition-colors">"HASH: 0x7F..."</span>
                </div>

                // Title
                <h3 class="text-xl font-bold text-zinc-100 group-hover:text-orange-500 transition-colors">
                    {"> "}{project.title}
                </h3>
                
                // Description (Terminal green/gray)
                <p class="text-zinc-400 mt-3 text-sm leading-relaxed border-l-2 border-zinc-800 pl-4 group-hover:border-orange-500/50 transition-all">
                    {project.description}
                </p>

                // Tech Stack (displayed as an array)
              <div class="flex flex-wrap gap-2 mt-6">
                    {project.tech_stack.into_iter().map(|tech| {
                        view! { 
                            <span class="text-[10px] uppercase bg-black/40 text-orange-200/80 px-2 py-1 border border-zinc-700/50">
                                {tech}
                            </span> 
                        }
                    }).collect_view()}
                </div>

                // "Execute" Button
               <a href=project.link target="_blank" class="mt-6 block text-center w-full py-3 rounded-sm btn-magma">
                    "[ IGNITE NODE ]"
                </a>
            </div>

            // Background "glitch" decoration
            <div class="absolute -right-4 -bottom-4 text-9xl font-black text-zinc-800/20 group-hover:text-orange-900/10 transition-colors select-none pointer-events-none">
                "0"{project.id}
            </div>
        </div>
    }
}