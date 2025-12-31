use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast; 

#[component]
pub fn ConsoleGuard() -> impl IntoView {
    Effect::new(move |_| {
        // 1. Clear Console & Log Security Message

        web_sys::console::clear();
        web_sys::console::log_1(&"// SYSTEM SECURED: ARCHITECTURE HIDDEN".into());

        if let Some(win) = web_sys::window() {
            
            // 2. Disable Right Click
            if let Some(doc) = win.document() {
                if let Some(body) = doc.body() {
                    let closure = Closure::wrap(Box::new(move |e: web_sys::Event| {
                        e.prevent_default();
                    }) as Box<dyn FnMut(_)>);

                    let _ = body.add_event_listener_with_callback(
                        "contextmenu",
                        closure.as_ref().unchecked_ref(),
                    );
                    
                    closure.forget();
                }
            }

            // 3. Disable F12 / DevTools Shortcuts
            let closure_keys = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
                let key = e.key();
                let code = e.key_code(); 
                
                // Block F12 (123), Ctrl+Shift+I, Ctrl+Shift+J, Ctrl+U
                if code == 123 || 
                   (e.ctrl_key() && e.shift_key() && (key == "I" || key == "J")) || 
                   (e.ctrl_key() && key == "U") 
                {
                    e.prevent_default();
                    e.stop_propagation();
                }
            }) as Box<dyn FnMut(_)>);

            let _ = win.add_event_listener_with_callback(
                "keydown",
                closure_keys.as_ref().unchecked_ref(),
            );
            
            closure_keys.forget();
        }
    });

    view! {
        <div style="display: none;"></div>
    }
}