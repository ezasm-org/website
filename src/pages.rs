use leptos::*;
use leptos_meta::*;

#[component]
pub fn Landing(cx: Scope) -> impl IntoView{
    view! {
        cx,
        <div class="flex dark:bg-gray-700 bg-slate-100 items-center">
            <section class="pt-5 mt-n3 pb-3">
            <div class="flex flex-row flex-wrap">
                <div class="flex basis-full md:basis-1/2 px-5 items-center">
                    <div class="">
                            <p class="tracking-wide leading-9 font-extrabold text-4xl dark:text-white text-black">
                            "Accessible, simplified, readable assembly code for students and teachers."
                            </p>
                            <div class="my-5">
                               <a href="/releases" class="px-6 py-3 text-blue-100 no-underline bg-blue-500 rounded hover:bg-blue-600 hover:underline hover:text-blue-200">
                                "Download"
                                </a>
                            </div>
                    </div>
                </div>
                <div class="flex basis-full md:basis-1/2 justify-center"> 
                    <img class="scale-75 md:scale-100 img-fluid px-0 box-shadow hover:-translate-y-2" src="public/ezasm_screenshot.png"/>
                </div>
            </div>
            </section>
        </div>
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
