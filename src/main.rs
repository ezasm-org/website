mod components;

use leptos::*;
use crate::components::*;

fn main() {
    mount_to_body(|cx| {
        view! { cx, <Navbar /> }
    });
}
