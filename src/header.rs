use leptos::prelude::*;
use leptos_router::hooks::use_location;

use crate::{icons::Icon, picture::Picture};

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

    pub fn aria_label(&self) -> &str {
        match self {
            MenuPage::Home => "Go to Home Page",
            MenuPage::Projects => "Find all of my Projects",
            MenuPage::About => "Get to know more about me",
            MenuPage::Contact => "Contact me",
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
    let desktop_pages = &[MenuPage::Home, MenuPage::Projects, MenuPage::About];

    let location = use_location();
    view! {

        <div class="px-8 md:px-20">

            <nav class="justify-center gap-4 text-slate-300 font-semibold text-xl py-5 hidden md:flex items-center">
                <a aria-label="Go to Home" href={MenuPage::Home.path()}> <SiteIcon /> </a>
                <div class="flex-grow" />
                {
                    desktop_pages.iter().map(|page| {
                        view! {
                            <a aria-label={page.aria_label()} href={page.path()}> <span class=("text-accent", move||location.pathname.get() == page.path())> {page.name()} </span> </a>
                        }
                    }).collect_view()
                }
                <div class="flex-grow" />

                <a aria-label={MenuPage::Contact.aria_label()} href={MenuPage::Contact.path()}> <Icon icon=crate::icons::Icons::Chat /> </a>
            </nav>


            <MobileMenu>
                <SiteIcon />
            </MobileMenu>
        </div>
    }
}

#[island]
fn MobileMenu(children: Children) -> impl IntoView {
    let mobile_pages = &[
        MenuPage::Home,
        MenuPage::Projects,
        MenuPage::About,
        MenuPage::Contact,
    ];

    let (location, set_location) = signal(String::new());
    Effect::new(move |_| {
        set_location.set(window().location().pathname().unwrap_or_default());
    });

    let (is_open, set_is_open) = signal(false);
    view! {
        <div class="flex py-6 md:hidden">
            <a aria-label={MenuPage::Home.aria_label()} href={MenuPage::Home.path()}> {children()} </a>
            <div class="flex-grow" />
            <button
                aria-label="Mobile Menu"
                on:click=move|_|{
                    set_is_open.set(!is_open.get_untracked());
                }
                class="text-xl"
            >
                {
                    move|| match is_open.get() {
                        true => view! {
                            <Icon icon=crate::icons::Icons::Close />
                        },
                        false => view! {
                            <Icon icon=crate::icons::Icons::Menu />
                        },
                    }
                }
            </button>
        </div>
        <nav
            class=move||format!("flex text-left flex-col justify-center gap-4 text-slate-300 font-semibold text-xl px-5 h-auto transition-all duration-500 {}",
            if is_open.get() {"max-h-96 visible opacity-100"}else {"max-h-0 invisible opacity-0"}
            )
        >
            {
                mobile_pages.iter().map(|page| {
                    view! {
                        <a aria-label={page.aria_label()} href={page.path()}> <span class=("text-accent", move||location.get() == page.path())> {page.name()} </span> </a>
                    }
                }).collect_view()
            }
            <div class="h-8 flex-shrink-0" />
        </nav>
    }
}

#[component]
fn SiteIcon() -> impl IntoView {
    view! {
        <Picture attr:class="h-10 w-auto" src="/assets/images/icon.png" alt="Site Icon" />
    }
}
