use leptos::prelude::*;

use crate::header::Header;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="min-h-dvh w-full flex flex-col p-10">
            <Header />
            <div class="flex-grow w-full flex flex-col justify-center items-stretch">
                <div class="text-slate-300 text-lg text-left">
                    "Hi, I'm Deep 👋"
                </div>
                <div class="h-6" />
                <h1 class="text-7xl font-bold text-left ">
                    <span class="text-accent"> "Software" </span>
                    <br/>
                    "Developer"
                </h1>
                <div class="h-8" />
                <h2 class="text-left text-xl text-slate-200" > "I'm a software developer from India."<br/>"I love solving problems that bring people a little closer." </h2>
            </div>
        </div>
    }
}
