use leptos::{either::Either, prelude::*, text_prop::TextProp};
use leptos_meta::{Meta, Title};
use leptos_router::hooks::use_location;

fn get_title(title: &str) -> String {
    format!("Deep Gaurav | {title}")
}

#[component]
pub fn SiteMeta(
    #[prop(into)] title: TextProp,
    #[prop(optional)] description: Option<TextProp>,
    #[prop(optional)] gen_img: Option<bool>,
) -> impl IntoView {
    let location = use_location();
    view! {
        <Title text=get_title(title.get().as_str()) />
        <Meta property="og:type" content="website" />
        <Meta property="og:title" content=get_title(title.get().as_str()) />
        <Meta property="og:title" content=format!("https://deepgaurav.com{}", location.pathname.get()) />
        {
            if gen_img.unwrap_or(true) {
                Either::Left(view! {
                    <Meta property="og:image" content="gen_og.jpg" />
                })
            }else{
                Either::Right(())
            }
        }
        {
            if let Some(description) = description {
                let desc = description.get().as_str().to_string();
                Either::Left(view! {
                    <Meta property="og:description" content={desc} />
                })
            }else{
                Either::Right(())
            }
        }
    }
}
