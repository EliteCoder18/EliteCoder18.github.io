use leptos::prelude::*;
use leptos::ev::SubmitEvent;
use leptos::leptos_dom::helpers::set_timeout;
use std::time::Duration;

#[component]
pub fn ContactForm() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (message, set_message) = signal(String::new());
    let (status, set_status) = signal("idle");

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        if status.get() == "sending" || status.get() == "success" { return; }
        set_status.set("sending");
        set_timeout(move || {
            set_status.set("success");
            set_name.set(String::new());
            set_email.set(String::new());
            set_message.set(String::new());
            set_timeout(move || { set_status.set("idle"); }, Duration::from_secs(3));
        }, Duration::from_millis(1500));
    };

    view! {
        <section class="w-full max-w-4xl mx-auto px-4 py-20 relative z-10">
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
                <div class="absolute -inset-1 bg-gradient-to-r from-orange-500/20 to-zinc-600/20 rounded-xl blur opacity-25 group-hover:opacity-50 transition duration-1000"></div>
                
                <div class="relative bg-zinc-900 border border-zinc-800 rounded-xl p-8 md:p-12 shadow-2xl">
                    <form on:submit=on_submit class="space-y-6 relative z-10">
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                            
                            // PLAIN ENGLISH LABELS
                            <div class="space-y-2">
                                <label class="text-sm font-semibold text-zinc-300 ml-1">"Name"</label>
                                <input
                                    type="text"
                                    required
                                    placeholder="Your_Name"
                                    class="w-full bg-zinc-950/50 border border-zinc-800 rounded-lg px-4 py-3 text-zinc-100 focus:border-orange-500 focus:ring-1 focus:ring-orange-500 transition-all"
                                    prop:value=name
                                    on:input=move |ev| set_name.set(event_target_value(&ev))
                                    disabled=move || status.get() != "idle"
                                />
                            </div>

                            <div class="space-y-2">
                                <label class="text-sm font-semibold text-zinc-300 ml-1">"Email Address"</label>
                                <input
                                    type="email"
                                    required
                                    placeholder="your_email_id@example.com"
                                    class="w-full bg-zinc-950/50 border border-zinc-800 rounded-lg px-4 py-3 text-zinc-100 focus:border-orange-500 focus:ring-1 focus:ring-orange-500 transition-all"
                                    prop:value=email
                                    on:input=move |ev| set_email.set(event_target_value(&ev))
                                    disabled=move || status.get() != "idle"
                                />
                            </div>
                        </div>

                        <div class="space-y-2">
                            <label class="text-sm font-semibold text-zinc-300 ml-1">"Message"</label>
                            <textarea
                                required
                                rows="5"
                                placeholder="How can I help you?"
                                class="w-full bg-zinc-950/50 border border-zinc-800 rounded-lg px-4 py-3 text-zinc-100 focus:border-orange-500 focus:ring-1 focus:ring-orange-500 transition-all resize-none"
                                prop:value=message
                                on:input=move |ev| set_message.set(event_target_value(&ev))
                                disabled=move || status.get() != "idle"
                            ></textarea>
                        </div>

                        <div class="pt-4 flex justify-end">
                            <button
                                type="submit"
                                disabled=move || status.get() != "idle"
                                class="bg-zinc-100 text-zinc-900 px-8 py-3 rounded-lg font-bold hover:bg-orange-500 hover:text-white transition-all duration-300 disabled:opacity-50 min-w-[140px]"
                            >
                                {move || match status.get() {
                                    "sending" => "Sending...",
                                    "success" => "Me`ssage Sent!",
                                    _ => "Send Message"
                                }}
                            </button>
                        </div>
                    </form>
                </div>
            </div>
        </section>
    }
}