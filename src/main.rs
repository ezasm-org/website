mod components;

use crate::components::*;
use leptos::*;

fn main() {
    mount_to_body(|cx| {
        view! { cx, <Navbar /> <Footer/>}
    });
}
