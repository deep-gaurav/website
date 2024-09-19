use leptos::prelude::*;

use crate::header::Header;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="min-h-dvh w-full flex flex-col p-20">
            <Header />
            <div class="flex-grow w-full flex  justify-stretch items-center">
                <div class="flex flex-col">
                    <div class="text-slate-300 text-lg text-left">
                        "Hi, I'm Deep 👋"
                    </div>
                    <div class="h-6" />
                    <h1 class="text-7xl font-bold text-left ">
                        <span class="text-accent"> "Software" </span>
                        <br/>
                        "Developer"
                    </h1>
                    <div class="h-8" />
                    <h2 class="text-left text-xl text-slate-300" > "I'm a software developer from India."<br/>"I love solving problems that bring people a little closer." </h2>
                    <div class="h-6" />
                    <div class="flex gap-4">
                        <button class="p-2 px-4 transition-colors bg-accent hover:bg-accent-dark rounded-md text-black font-medium"> "Get In Touch" </button>
                        <button class="p-2 px-4 border transition-colors border-white hover:bg-white hover:text-black rounded-md font-medium box-border"> "Browse Projects" </button>

                    </div>
                </div>

                <div class="flex-grow" />
                <div class="p-10 border border-solid border-accent rounded-full w-[30%]">
                    <div class="rounded-full relative overflow-hidden w-full aspect-square">
                        <img class="inline h-full w-full object-cover" src="/assets/images/deep.webp" />
                    </div>
                </div>
            </div>
        </div>
    }
}
