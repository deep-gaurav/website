use leptos::{prelude::*, text_prop::TextProp};

#[component]
pub fn Picture(#[prop(into)] src: TextProp) -> impl IntoView {
    let src = src.get().as_str().to_string();
    view! {
        <img
            data-gen="true"
            src={src}
        />
    }
}
