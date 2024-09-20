use std::borrow::Cow;

use leptos::prelude::*;

use crate::header::Header;

pub struct Project {
    pub name: Cow<'static, str>,
    pub cover_image: Cow<'static, str>,
    pub short_description: Cow<'static, str>,
}

pub const PROJECTS: &[Project] = &[
    Project {
        name: std::borrow::Cow::Borrowed("BillDivide"),
        cover_image: std::borrow::Cow::Borrowed("/assets/images/projects/billdivide/cover.png"),
        short_description: std::borrow::Cow::Borrowed(
            r#"BillDivide is a simple and easy-to-use app that lets you split bills with friends, family, or anyone else. Whether you're dining out, going on vacation, or just sharing household expenses, BillDivide makes it easy to track who owes what and settle up quickly and easily."#,
        ),
    },
    Project {
        name: std::borrow::Cow::Borrowed("TVMate"),
        cover_image: std::borrow::Cow::Borrowed("/assets/images/projects/tvmate/cover.png"),
        short_description: std::borrow::Cow::Borrowed(
            r#"A nostalgic video-syncing app with CRT TV aesthetics, allowing users to watch movies together remotely while chatting in a shared virtual room."#,
        ),
    },
    Project {
        name: std::borrow::Cow::Borrowed("SelfCloud"),
        cover_image: std::borrow::Cow::Borrowed("/assets/images/projects/selfcloud/cover.png"),
        short_description: std::borrow::Cow::Borrowed(
            r#"A self-hosted platform that simplifies project deployment by managing containers, domains, and SSL certificates, allowing easy routing of traffic to specific project ports through a reverse proxy system."#,
        ),
    },
];

#[component]
pub fn ProjectsPage() -> impl IntoView {
    view! {
        <div class="min-h-dvh w-full flex flex-col px-20">
            <Header />

        </div>
    }
}
