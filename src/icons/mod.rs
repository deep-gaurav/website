use leptos::prelude::*;

pub enum Icons {
    Mail,
    Github,
    LinkedIn,
}

impl Icons {
    pub fn svg(&self) -> &'static str {
        match self {
            Icons::Mail => include_str!("mail.svg"),
            Icons::Github => include_str!("linkedin.svg"),
            Icons::LinkedIn => include_str!("github.svg"),
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
