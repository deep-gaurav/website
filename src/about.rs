use leptos::prelude::*;

use crate::{footer::Footer, header::Header, icons::Icon};

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <Header />
        <div class="flex flex-col px-20">
            <div class="h-20" />
            <h1 class="text-7xl font-bold text-left"> "About me"<span class="text-accent">"."</span></h1>
            <div class="h-8" />
            <h2 class="text-left text-xl text-slate-300" > "Tech enthusiast building connective solutions, one app at a time. Passionate about crafting innovative experiences that bring people closer together." </h2>
            <div class="h-10" />
            <div class="flex">
                <div class="flex flex-col flex-shrink-0 flex-grow basis-0 relative w-1">
                    <h3 class="text-2xl font-semibold"> "My Stack" </h3>
                    <TechStack />
                </div>
                <div class="flex flex-col flex-shrink-0 flex-grow basis-0 relative w-1">
                    <h3 class="text-2xl font-semibold"> "My Stack" </h3>
                    <TechStack />
                </div>
            </div>
        </div>
        <div class="h-20" />

        <Footer />
    }
}

#[component]
fn TechStack() -> impl IntoView {
    let frontend_stack = [
        "HTML/CSS",
        "Leptos (Rust)",
        "Flutter",
        "Tailwind",
        "ReactJS",
    ];

    view! {
        <div class="bg-white/40 h-20 flex flex-col justify-center overflow-hidden marquee">
            <div class="flex marquee-content gap-4">
                {
                    frontend_stack.iter().chain(frontend_stack.iter()).map(
                        |tech| view! {
                            <span class="bg-white text-black p-2 rounded text-nowrap"> {*tech} </span>
                            <Icon icon=crate::icons::Icons::Star />
                        }
                    ).collect_view()
                }
            </div>
        </div>
    }
}
