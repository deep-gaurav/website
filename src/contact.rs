use leptos::prelude::*;
use leptos_router::components::A;

use crate::{footer::Footer, header::Header, icons::Icon};

#[component]
pub fn ContactPage() -> impl IntoView {
    view! {
        <div class="min-h-dvh w-full flex flex-col px-20">
            <Header />

            <div class="flex-grow w-full flex  justify-stretch items-center">
                <div class="flex flex-col">
                    <h1 class="text-5xl font-bold text-left ">
                        "Get In Touch"
                        <span class="text-accent"> "." </span>
                    </h1>
                    <div class="h-8" />
                    <h2 class="text-left text-xl text-slate-300" > "Looking to partner or work together?"<br/>"Reach out over my email and I'll get back to you in the next 48 hours." </h2>
                    <div class="h-6" />
                    <A href="mailto:deepgauravraj@gmail.com" {..view! {< {..} class="text-slate-300 text-lg flex gap-4 items-center hover:text-accent" />}}>
                        <div class="p-2 rounded-full border-slate-400 border"> <Icon icon=crate::icons::Icons::Mail /> </div>
                        <div> "deepgauravraj@gmail.com" </div>
                    </A>
                </div>

                <div class="flex-grow" />
                <div class="p-10 border border-solid border-accent rounded-full w-[30%]">
                    <div class="rounded-full relative overflow-hidden w-full aspect-square">
                        <img class="inline h-full w-full object-cover" src="/assets/images/deep.webp" />
                    </div>
                </div>
            </div>
        </div>

        <div class="h-20" />

        <Footer />
    }
}
