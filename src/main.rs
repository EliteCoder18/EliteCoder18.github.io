// Declare the modules we created
mod models;
mod components;
mod app;

use leptos::prelude::*;
use app::App;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}