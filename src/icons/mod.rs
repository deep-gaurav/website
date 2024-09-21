use leptos::prelude::*;

pub enum Icons {
    Mail,
    Github,
    LinkedIn,
    Star,
    Menu,
    Close,
    GooglePlay,
    Web,
}

impl Icons {
    pub fn svg(&self) -> &'static str {
        match self {
            Icons::Mail => include_str!("mail.svg"),
            Icons::Github => include_str!("linkedin.svg"),
            Icons::LinkedIn => include_str!("github.svg"),
            Icons::Star => include_str!("star.svg"),
            Icons::Menu => include_str!("menu.svg"),
            Icons::Close => include_str!("close.svg"),
            Icons::GooglePlay => include_str!("google-play.svg"),
            Icons::Web => include_str!("globe.svg"),
        }
    }
}

#[component]
pub fn Icon(icon: Icons) -> impl IntoView {
    view! {
        <span inner_html=icon.svg()>
        </span>
    }
}
