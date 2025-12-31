// Declare the modules we created
mod models;
mod components;
mod app;
#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => {
        #[cfg(debug_assertions)]
        leptos::logging::log!($($t)*);
    }
}
use leptos::prelude::*;
use app::App;

fn main() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
