use leptos::{either::Either, prelude::*, text_prop::TextProp};
use leptos_meta::{Meta, Title};
use leptos_router::hooks::use_location;

fn get_title(title: &str) -> String {
    format!("Deep Gaurav | {title}")
}

#[component]
pub fn SiteMeta(
    #[prop(into)] title: TextProp,

    #[prop(into)] description: TextProp,
    #[prop(optional)] gen_img: Option<bool>,
) -> impl IntoView {
    let location = use_location();
    let desc = description.get().as_str().to_string();

    view! {
        <Title text=get_title(title.get().as_str()) />
        <Meta property="og:type" content="website" />
        <Meta property="og:title" content=get_title(title.get().as_str()) />
        <Meta property="og:description" content={desc} />
        <Meta property="og:url" content=format!("https://deepgaurav.com{}", if location.pathname.get().starts_with("/"){
            location.pathname.get()
        }else{
            format!("/{}",location.pathname.get())
        }) />
        {
            let path = if location.pathname.get().starts_with("/"){
                location.pathname.get()
            }else{
                format!("/{}",location.pathname.get())
            };
            let slash = if path.ends_with("/"){
                ""
            }else {
                "/"
            };
            if gen_img.unwrap_or(true) {
                Either::Left(view! {
                    <Meta property="og:image" content=format!("https://deepgaurav.com{}{}og_image.jpg", path,slash) />
                })
            }else{
                Either::Right(())
            }
        }

    }
}
