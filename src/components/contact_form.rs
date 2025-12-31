use leptos::prelude::*;
use leptos::ev::SubmitEvent;
use leptos::task::spawn_local; // Needed for async
use serde::{Deserialize, Serialize}; // Needed for data handling

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContactForm {
    name: String,
    email: String,
    message: String,
}

#[component]
pub fn Contact() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (message, set_message) = signal(String::new());
    let (status, set_status) = signal("idle");

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        
        // Prevent double submission
        if status.get() == "sending" || status.get() == "success" { return; }
        
        set_status.set("sending");

        spawn_local(async move {
            let client = reqwest::Client::new();
            let payload = ContactForm {
                name: name.get(),
                email: email.get(),
                message: message.get(),
            };

            // 👇 REPLACE THIS WITH YOUR REAL FORMSPREE ID
            let res = client.post("https://formspree.io/f/mvzoepgp")
                .header("Accept", "application/json")
                .form(&payload) // Using .form() for best compatibility
                .send()
                .await;

            match res {
                Ok(response) => {
                    if response.status().is_success() {
                        set_status.set("success");
                        // Clear inputs
                        set_name.set(String::new());
                        set_email.set(String::new());
                        set_message.set(String::new());
                        
                        // Optional: Reset button after 3 seconds
                        leptos::leptos_dom::helpers::set_timeout(move || {
                            set_status.set("idle"); 
                        }, std::time::Duration::from_secs(3));
                    } else {
                        set_status.set("error");
                    }
                }
                Err(_) => {
                    set_status.set("error");
                }
            }
        });
    };

    view! {
        <section class="w-full max-w-4xl mx-auto px-4 py-20 relative z-10" id="contact">
            // HUMAN READABLE HEADER
            <div class="mb-12 text-center">
                <h2 class="text-4xl md:text-5xl font-bold text-zinc-100 mb-4 tracking-tight">
                    "Get In Touch"
                </h2>
                <div class="h-1 w-24 bg-orange-500 mx-auto rounded-full"></div>
                <p class="mt-4 text-zinc-400 text-lg max-w-lg mx-auto">
                    "Have a project in mind or want to discuss modern web tech? Drop me a message below."
                </p>
            </div>

            <div class="relative group">
                // Background Glow Effect
                <div class="absolute -inset-1 bg-gradient-to-r from-orange-500/20 to-zinc-600/20 rounded-xl blur opacity-25 group-hover:opacity-50 transition duration-1000"></div>
                
                <div class="relative bg-zinc-900 border border-zinc-800 rounded-xl p-8 md:p-12 shadow-2xl">
                    
                    // SUCCESS MESSAGE OVERLAY
                    {move || if status.get() == "success" {
                        view! {
                            <div class="absolute inset-0 z-20 flex flex-col items-center justify-center bg-zinc-900/95 backdrop-blur-sm rounded-xl animate-fade-in">
                                <div class="text-green-500 text-6xl mb-4">"✓"</div>
                                <h3 class="text-2xl font-bold text-white mb-2">"Message Sent!"</h3>
                                <p class="text-zinc-400">"I'll get back to you soon."</p>
                            </div>
                        }.into_any()
                    } else {
                        view! { <span/> }.into_any()
                    }}

                    <form on:submit=on_submit class="space-y-6 relative z-10">
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                            
                            // Name Input
                            <div class="space-y-2">
                                <label class="text-sm font-semibold text-zinc-300 ml-1">"Name"</label>
                                <input
                                    type="text"
                                    required
                                    placeholder="Your Name"
                                    class="w-full bg-zinc-950/50 border border-zinc-800 rounded-lg px-4 py-3 text-zinc-100 focus:border-orange-500 focus:ring-1 focus:ring-orange-500 transition-all outline-none"
                                    prop:value=name
                                    on:input=move |ev| set_name.set(event_target_value(&ev))
                                    disabled=move || status.get() == "sending"
                                />
                            </div>

                            // Email Input
                            <div class="space-y-2">
                                <label class="text-sm font-semibold text-zinc-300 ml-1">"Email Address"</label>
                                <input
                                    type="email"
                                    required
                                    placeholder="you@example.com"
                                    class="w-full bg-zinc-950/50 border border-zinc-800 rounded-lg px-4 py-3 text-zinc-100 focus:border-orange-500 focus:ring-1 focus:ring-orange-500 transition-all outline-none"
                                    prop:value=email
                                    on:input=move |ev| set_email.set(event_target_value(&ev))
                                    disabled=move || status.get() == "sending"
                                />
                            </div>
                        </div>

                        // Message Input
                        <div class="space-y-2">
                            <label class="text-sm font-semibold text-zinc-300 ml-1">"Message"</label>
                            <textarea
                                required
                                rows="5"
                                placeholder="How can I help you?"
                                class="w-full bg-zinc-950/50 border border-zinc-800 rounded-lg px-4 py-3 text-zinc-100 focus:border-orange-500 focus:ring-1 focus:ring-orange-500 transition-all resize-none outline-none"
                                prop:value=message
                                on:input=move |ev| set_message.set(event_target_value(&ev))
                                disabled=move || status.get() == "sending"
                            ></textarea>
                        </div>

                        // Submit Button
                        <div class="pt-4 flex justify-end">
                            <button
                                type="submit"
                                disabled=move || status.get() == "sending"
                                class="bg-zinc-100 text-zinc-900 px-8 py-3 rounded-lg font-bold hover:bg-orange-500 hover:text-white transition-all duration-300 disabled:opacity-50 disabled:cursor-not-allowed min-w-[140px]"
                            >
                                {move || match status.get() {
                                    "sending" => "Sending...",
                                    "error" => "Try Again",
                                    _ => "Send Message"
                                }}
                            </button>
                        </div>
                        
                        // Error Text (Only visible on error)
                        {move || if status.get() == "error" {
                            view! { <p class="text-red-500 text-sm text-right">"Something went wrong. Please check your connection."</p> }.into_any()
                        } else {
                            view! { <span/> }.into_any()
                        }}
                    </form>
                </div>
            </div>
        </section>
    }
}