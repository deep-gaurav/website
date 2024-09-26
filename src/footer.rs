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
        <footer class="flex flex-col px-8 md:px-20">
            <div class="flex flex-col md:flex-row gap-4 items-center">
                <nav class="flex justify-center gap-4 text-slate-300 font-semibold text-lg">
                    {
                        pages.iter().map(|page| {
                            view! {
                                <A
                                    attr:aria-label=page.aria_label()
                                    href={page.path()}> <span class=("text-accent", move||location.pathname.get() == page.path())> {page.name()} </span> </A>
                            }
                        }).collect_view()
                    }
                </nav>
                <div class="flex-grow"/>
                <div class="flex gap-4">
                    <A attr:aria-label="Mail me" href="mailto:deepgauravraj@gmail.com"> <Icon {..view! { <{..} class="hover:text-accent" />}}  icon=crate::icons::Icons::Mail /> </A>
                    <A attr:aria-label="My Github" href="https://github.com/deep-gaurav"> <Icon {..view! { <{..} class="hover:text-accent" />}} icon=crate::icons::Icons::Github /> </A>
                    <A attr:aria-label="My LinkedIn" href="https://www.linkedin.com/in/deepgauravraj/"> <Icon {..view! { <{..} class="hover:text-accent" />}}  icon=crate::icons::Icons::LinkedIn /> </A>
                </div>
            </div>
            <div class="h-10 flex-shrink-0" />
            <div class="flex flex-col md:flex-row items-center gap-4">
                <div class="flex flex-col gap-4" >
                    <div class="text-xl font-semibold"> "Interested in working together" <span class="text-accent"> "?" </span> </div>
                    <div class="flex gap-4">
                        <A attr:aria-label={MenuPage::Contact.aria_label()} href={MenuPage::Contact.path()} {..view! {< {..} class="p-2 px-4 transition-colors bg-accent hover:bg-accent-dark rounded-md text-black font-medium" />}} > "Get In Touch" </A>
                        <A attr:aria-label={MenuPage::Projects.aria_label()} href={MenuPage::Projects.path()} {..view! {< {..} class="p-2 px-4 border transition-colors border-white hover:bg-white hover:text-black rounded-md font-medium box-border" />}}> "Browse Projects" </A>
                    </div>
                </div>
                <div class="flex-grow" />
                <div class="flex flex-col text-slate-300">
                    <div class="text-lg"> "©2024 All Rights Reserved." </div>
                    <a aria-label="Website github source" href="https://github.com/deep-gaurav/website/" class="text-base flex gap-2 justify-center md:justify-end"> <Icon icon=crate::icons::Icons::Github /> "Website Source" </a>
                </div>
            </div>
        </footer>
        <div class="h-12 flex-shrink-0" />
    }
}
