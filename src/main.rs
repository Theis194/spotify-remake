#[warn(non_snake_case)]

mod app;
mod test;
mod ui_elements;
mod pages;

use app::*;
use test::*;
use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <Main/>
        }
    })
}
