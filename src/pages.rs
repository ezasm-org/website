use leptos::*;
use leptos_meta::*;

#[component]
pub fn Landing(cx: Scope) -> impl IntoView{
    view! {
        cx,
        <p> "Landing Page"</p>
    }
}

#[component]
pub fn Download(cx: Scope) -> impl IntoView{
    view! {
        cx,
        <p> "Download Page"</p>
    }
}

#[component]
pub fn Documentation(cx: Scope) -> impl IntoView{
    view! {
        cx,
        <p> "Doc Page"</p>
    }
}

#[component]
pub fn About(cx: Scope) -> impl IntoView{
    view! {
        cx,
        <p> "About Page"</p>
    }
}
