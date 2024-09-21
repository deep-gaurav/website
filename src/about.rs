use leptos::prelude::*;
use leptos_meta::Script;

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
            <div class="flex gap-10 items-stretch">
                <div class="flex flex-col flex-shrink-0 flex-grow-[1]  basis-0 relative w-1">
                    <h3 class="text-2xl font-semibold text-left"> "My Stack" </h3>
                    <div class="h-4" />
                    <TechStack />
                </div>
                <div class="flex flex-col flex-shrink-0 flex-grow-[2] basis-0 relative w-1">
                    <h3 class="text-2xl font-semibold text-left"> "My Place" </h3>
                    <div class="h-4" />
                    <Location />
                </div>
            </div>
        </div>
        <div class="h-20" />

        <Footer />
    }
}

#[component]
fn TechStack() -> impl IntoView {
    let frontend_stack: &[&'static str] = &[
        "HTML/CSS",
        "Leptos (Rust)",
        "Flutter",
        "Tailwind",
        "ReactJS",
        "Android (Kotlin/Java)",
        "iOS (Swift)",
    ];

    let backend_stack: &[&'static str] = &[
        "Axum (Rust)",
        "Leptos (Rust)",
        "GraphQL",
        "REST API",
        "WebSocket",
        "WebRTC",
        "WebTransport",
    ];

    let other_stack: &[&'static str] = &[
        "SQLite",
        "Postgres",
        "Docker",
        "Podman",
        "Litestream",
        "WebTransport",
    ];

    let stacks = [frontend_stack, backend_stack, other_stack];

    view! {
        <div class="bg-white/10 py-20 flex flex-col justify-center overflow-hidden marquee rounded-md gap-4 flex-grow">
            {
                stacks.iter().enumerate().map(|(i,stack)|{
                    view! {
                        <div class=format!("flex marquee-content gap-4 {}", if i%2==0 {"marquee-forward"}else {"marquee-rev"})>
                            {
                                stack.iter().chain(stack.iter()).map(
                                    |tech| view! {
                                        <span class="bg-slate-300 text-black p-2 rounded text-nowrap"> {*tech} </span>
                                        <Icon {..view! {< {..} class="text-accent" />}} icon=crate::icons::Icons::Star />
                                    }
                                ).collect_view()
                            }
                        </div>
                    }
                }).collect_view()
            }
        </div>
    }
}

#[component]
fn Location() -> impl IntoView {
    view! {
        <div class="bg-white/10 overflow-hidden rounded-md flex justify-center">
            <div class="aspect-square w-full max-w-[400px]">
                <canvas id="cobe" class="w-full h-full cursor-grab" />
                <Script defer="true" type_="module" src="/assets/scripts/cobe.js" />
            </div>
        </div>
    }
}
