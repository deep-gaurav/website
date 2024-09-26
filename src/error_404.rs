use leptos::prelude::*;

use crate::{
    footer::Footer,
    header::{Header, MenuPage},
    utils::title::SiteMeta,
};

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <SiteMeta title="Not Found" description="Looks like page you're trying to vising doesnt exist" />
        <Header />

        <div id="og-image" class="flex-grow w-full flex flex-col px-8 md:px-20 items-center justify-center">
            <h1 class="text-5xl md:text-5xl font-bold text-center"> "Page Not Found" </h1>
            <div class="h-4 flex-shrink-0" />
            <div class="text-slate-300 text-lg text-center">
                "Looks like page you're trying to vising does not exist"
            </div>
            <div class="h-4 flex-shrink-0" />
            <a attr:aria-label={MenuPage::Home.aria_label()} href={MenuPage::Home.path()} {..view! {< {..} class="p-2 px-4 transition-colors bg-accent hover:bg-accent-dark rounded-md text-black font-medium" />}} > "Go Home" </a>


        </div>

        <Footer />
    }
}
