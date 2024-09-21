use leptos::{either::Either, prelude::*};
use leptos_router::{components::A, hooks::use_location};

use crate::icons::Icon;

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

#[island]
pub fn Header() -> impl IntoView {
    let (is_mobile, set_is_mobile) = signal(true);
    let handle_size_change = move || {
        if let Ok(width) = window().inner_width() {
            if let Some(width) = width.as_f64() {
                let new_is_mobile = width < 768.0;
                if is_mobile.get_untracked() != new_is_mobile {
                    set_is_mobile.set(new_is_mobile)
                }
            }
        }
    };
    Effect::new(move |_| {
        handle_size_change();
    });
    let handle = window_event_listener(leptos::ev::resize, move |_| handle_size_change());
    on_cleanup(move || handle.remove());

    view! {
        {
            move|| match is_mobile.get() {
                true => Either::Left(view! {
                    <MobileHeader />
                }),
                false => Either::Right(view! {
                    <DesktopHeader />
                }),
            }
        }
    }
}

#[island]
fn MobileHeader() -> impl IntoView {
    let pages = &[
        MenuPage::Home,
        MenuPage::Projects,
        MenuPage::About,
        MenuPage::Contact,
    ];
    let location = use_location();
    let (is_open, set_is_open) = signal(false);
    view! {
        <div class="flex px-4 py-6">
            <div class="flex-grow" />
            <button
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
                pages.iter().map(|page| {
                    view! {
                        <A href={page.path()}> <span class=("text-accent", move||location.pathname.get() == page.path())> {page.name()} </span> </A>
                    }
                }).collect_view()
            }
            <div class="h-8" />
        </nav>
    }
}

#[component]
fn DesktopHeader() -> impl IntoView {
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
