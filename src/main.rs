// Declare the modules we created
mod app;
mod components;
mod models;
#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => {
        #[cfg(debug_assertions)]
        leptos::logging::log!($($t)*);
    }
}
use app::App;
use leptos::prelude::*;

fn main() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
