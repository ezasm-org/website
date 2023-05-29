use leptos::*;
use leptos_meta::*;

#[component]
pub fn Navbar(cx: Scope) -> impl IntoView{
    view!{cx,
        <nav class="bg-white border-gray-200 dark:border-b dark:bg-gray-700 bg-slate-100">
            <div class="max-w-screen-xl flex flex-wrap items-center justify-between mx-auto p-4">
                <a href="/" class="flex items-center">
                    <img  src="public/EzASM.svg" class="object-contain dark:invert p-3 h-16" alt="EzASM Logo"/>
                    <span class="self-center text-2xl font-semibold whitespace-nowrap dark:text-white">{"EzASM"}</span>
                </a>
            <button data-collapse-toggle="navbar-default" type="button" class="inline-flex items-center p-2 ml-3 text-sm text-gray-500 rounded-lg md:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600" aria-controls="navbar-default" aria-expanded="false">
                <span class="sr-only">{"Open main menu"}</span>
                <svg class="w-6 h-6" aria-hidden="true" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M3 5a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 15a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z" clip-rule="evenodd"></path></svg>
            </button> //Responsive for mobile. Does not work
            <div class="hidden w-full md:block md:w-auto" id="navbar-default">
                <ul class="font-medium flex flex-col p-4 md:p-0 mt-4 md:flex-row md:space-x-8 md:mt-0 md:border-0 bg-transparent">
                    <li>
                        <NavbarComponent href="/" text="Home"/>
                    </li>
                    <li>
                        <NavbarComponent href="/about" text="About"/>
                    </li>
                    <li>
                        //TODO add a dropdown here
                        <NavbarComponent href="/docs" text="Documentation"/>
                    </li>
                    <li>
                        <NavbarComponent href="/releases" text="Download"/>
                    </li>
                </ul>
            </div>
            </div>
        </nav>
    }
}

#[component]
pub fn NavbarComponent(cx: Scope,
                       #[prop(into)]
                       href:String,
                       #[prop(into)]
                       text:String) -> impl IntoView{
    view!{cx, 
        <a href={href} class="block py-2 pl-3 pr-4 text-white bg-blue-700 rounded md:bg-transparent md:text-blue-700 md:p-0 dark:text-white md:dark:text-blue-300" aria-current="page">{text}</a>
    }
}

#[component]
pub fn Footer(cx: Scope) -> impl IntoView{
    view! {cx,
        <footer class="bg-gray-700 dark:bg-slate-100 flex">
            <div class="bg-transparent h-40 w-full gap-4 content-center text-center">
                <div class="w-full flex flex-wrap items-center mx-auto max-w-screen-xl">
                    <a class= "items-center mx-auto max-w-screen-xl" href="https://github.com/ezasm-org">
                        <img  src="public/EzASM.svg" class="object-contain invert dark:filter-none mb-2 p-3 h-16" alt="EzASM Logo"/>
                    </a>
                </div>
                <div>
                <a href="https://new.rcos.io" class="inline-flex"> <FootNote text="An RCOS Project"/> </a>
                </div>
                <br/>
                <div>
                <a href="https://github.com/ezasm-org/EzASM/blob/main/LICENSE" class="inline-flex"> <FootNote text="All code involved with EzASM is under the MIT License"/> </a>
                </div>
            </div>
        </footer>
    }
}

#[component]
pub fn FootNote(cx: Scope,
                      #[prop(into)]
                      text:String) -> impl IntoView{
    view!{cx,
        <p class="dark:text-black text-white font-mono">{text}</p>
    }
}
