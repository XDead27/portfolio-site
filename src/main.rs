use app::*;
use leptos::{logging, mount};

mod app;
mod components;
mod data;
mod utils;

pub fn main() {
    console_error_panic_hook::set_once();
    logging::log!("csr mode - mounting to body");
    mount::mount_to_body(App);
}
