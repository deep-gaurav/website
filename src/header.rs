use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_location};

pub enum MenuPage {
    Home,
    Projects,
    About,
    Contact,
}

impl MenuPage {
    pub fn name(&self) -> &str {
        match self {
            MenuPage::Home => "Home",
            MenuPage::Projects => "Projects",
            MenuPage::About => "About",
            MenuPage::Contact => "Contact",
        }
    }

    pub fn path(&self) -> &str {
        match self {
            MenuPage::Home => "/",
            MenuPage::Projects => "/projects/",
            MenuPage::About => "/about/",
            MenuPage::Contact => "/contact/",
        }
    }
}

#[component]
pub fn Header() -> impl IntoView {
    let pages = &[MenuPage::Home, MenuPage::Projects, MenuPage::About];
    let location = use_location();
    view! {
        <nav class="flex justify-center gap-4 text-slate-300 font-semibold text-xl p-5">
            {
                pages.iter().map(|page| {
                    view! {
                        <A href={page.path()}> <span class=("text-accent", move||location.pathname.get() == page.path())> {page.name()} </span> </A>
                    }
                }).collect_view()
            }
        </nav>
    }
}
