use leptos::prelude::*;
use crate::models::Project;
use crate::components::project_card::ProjectCard;

#[component]
pub fn Projects() -> impl IntoView {

    let projects = vec![
        Project {
            id: 1,
            title: "LearnXTrade".into(),
            description: "An interactive platform facilitating community-driven skill exchange through practical lessons, challenges, and peer collaboration.".into(),
            tech_stack: vec!["JavaScript".into(), "CSS".into(), "Python".into(), "Shell".into()],
            link: "https://github.com/EliteCoder18/LearnXtrade".into(), // Update this specific repo link
        },
        Project {
            id: 2,
            title: "Encryted Log Storage".into(),
            description: "Secure Python pipeline for immutable log archival that encrypts system logs into PDFs, creates decentralized backups via IPFS, and anchors verifiable proofs on the blockchain.".into(),
            tech_stack: vec!["Python".into()],
            link: "https://github.com/EliteCoder18/EncryptedLogStorage".into(),
        },
        Project {
            id: 3,
            title: "Proofnest".into(),
            description: "A secure, blockchain-powered platform designed to help creators, professionals, and organizations register and verify digital content with authenticity and trust.".into(),
            tech_stack: vec!["JavaScript".into(), "Rust".into()],
            link: "https://github.com/EliteCoder18/Proofnest".into(),
        },
        Project {
            id: 4,
            title: "HostelHustle".into(),
            description: "Management platform for 4000+ students & 26+ events. Streamlined scheduling and resource allocation for TIET's largest fest.".into(),
            tech_stack: vec!["JavaScript".into(), "Go".into(), "MySQL".into()],
            link: "https://github.com/EliteCoder18/Hostel-Hustle".into(),
        },
        Project {
            id: 5,
            title: "Journey To Rustacean".into(),
            description: "A hands-on archive documenting my evolution into a Rustacean through daily commits, practical experiments, and progressive projects.".into(),
            tech_stack: vec!["Rust".into()],
            link: "https://github.com/EliteCoder18/journey-to-rustacean".into(),
        },
    ];

    view! {
        <section id="projects" class="max-w-6xl mx-auto px-6 py-24">
            <div class="flex items-center gap-4 mb-12">
                <div class="h-px bg-zinc-800 flex-1"></div>
                <h2 class="text-2xl font-bold text-orange-500 uppercase tracking-widest">"// Mined_Blocks"</h2>
                <div class="h-px bg-zinc-800 flex-1"></div>
            </div>
            
            <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                <For
                    each=move || projects.clone()
                    key=|p| p.id
                    children=|p| view! { <ProjectCard project=p /> }
                />
            </div>
        </section>
    }
}