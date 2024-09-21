use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_location};

use crate::{header::MenuPage, icons::Icon};

#[component]
pub fn Footer() -> impl IntoView {
    let pages = &[
        MenuPage::Home,
        MenuPage::Projects,
        MenuPage::About,
        MenuPage::Contact,
    ];
    let location = use_location();

    view! {
        <footer class="flex flex-col px-20">
            <div class="flex flex-col md:flex-row">
                <nav class="flex justify-center gap-4 text-slate-300 font-semibold text-lg">
                    {
                        pages.iter().map(|page| {
                            view! {
                                <A href={page.path()}> <span class=("text-accent", move||location.pathname.get() == page.path())> {page.name()} </span> </A>
                            }
                        }).collect_view()
                    }
                </nav>
                <div class="flex-grow"/>
                <div class="flex gap-4">
                    <A href="mailto:deepgauravraj@gmail.com"> <Icon {..view! { <{..} class="hover:text-accent" />}}  icon=crate::icons::Icons::Mail /> </A>
                    <A href="https://github.com/deep-gaurav"> <Icon {..view! { <{..} class="hover:text-accent" />}} icon=crate::icons::Icons::Github /> </A>
                    <A href="https://www.linkedin.com/in/deepgauravraj/"> <Icon {..view! { <{..} class="hover:text-accent" />}}  icon=crate::icons::Icons::LinkedIn /> </A>
                </div>
            </div>
            <div class="h-10" />
            <div class="flex flex-col md:flex-row">
                <div class="flex flex-col gap-4" >
                    <div class="text-xl font-semibold"> "Interested in working together" <span class="text-accent"> "?" </span> </div>
                    <div class="flex gap-4">
                        <A href={MenuPage::Contact.path()} {..view! {< {..} class="p-2 px-4 transition-colors bg-accent hover:bg-accent-dark rounded-md text-black font-medium" />}} > "Get In Touch" </A>
                        <A href={MenuPage::Projects.path()} {..view! {< {..} class="p-2 px-4 border transition-colors border-white hover:bg-white hover:text-black rounded-md font-medium box-border" />}}> "Browse Projects" </A>
                    </div>
                </div>
                <div class="flex-grow" />
                <div class="flex flex-col text-slate-300">
                    <div class="text-lg"> "©2024 All Rights Reserved." </div>
                    <div class="text-base"> "Made with 💜 by Deep Gaurav" </div>
                </div>
            </div>
        </footer>
        <div class="h-12" />
    }
}
