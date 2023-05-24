mod components;
mod pages;

use crate::components::*;
use crate::pages::*;
use leptos::*;
use leptos_router::*;

fn main() {
    mount_to_body(|cx| {
        view! { cx,
            <div id="root">
                <Router>
                    <Navbar/>
                        <Routing/>
                    <Footer/>
                </Router>
            </div>
        }
    });
}

#[component]
fn Routing(cx: Scope) -> impl IntoView   {
    view! {cx, 
        <Routes>
            <Route path=""
            view= move |cx| view!{cx, <Landing/>}
        />
            <Route path="about"
            view= move |cx| view!{cx, <About/>}
        />
            <Route path="releases"
            view= move |cx| view!{cx, <Download/>}
        />
            <Route path="docs"
            view= move |cx| view!{cx, <Documentation/>}
        />
            </Routes>
    }
}
