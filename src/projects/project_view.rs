use leptos::prelude::*;

use crate::{footer::Footer, header::Header};

use super::ProjectData;

#[component]
pub fn ProjectView(project: ProjectData) -> impl IntoView {
    view! {
        <div class="min-h-svh w-full flex flex-col px-8 md:px-20">
            <Header />

            <div class="h-10 md:h-20" />
            <h1 class="text-5xl md:text-7xl font-bold text-left"> {project.title} </h1>
            <div class="h-8" />
            <h2 class="text-left text-xl text-slate-300" > "From social apps to cloud solutions: a showcase of innovative projects solving real-world problems" </h2>

            <div class="h-20" />
            <div class="text-left text-slate-300" inner_html={project.html} />
        </div>

        <div class="h-20" />
        <Footer />
    }
}
