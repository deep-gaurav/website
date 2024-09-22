use leptos::{either::Either, prelude::*};

use crate::{footer::Footer, header::Header, icons::Icon};

use super::ProjectData;

#[component]
pub fn ProjectView(project: ProjectData) -> impl IntoView {
    view! {
        <div class="min-h-svh w-full flex flex-col px-8 md:px-20">
            <Header />

            <div class="h-10 md:h-20" />
            <h1 class="text-5xl md:text-7xl font-bold text-left"> {project.title} <span class="text-accent"> "." </span> </h1>
            <div class="h-8" />
            <h2 class="text-left text-xl text-slate-300" > {project.tagline} </h2>
            <div class="h-20" />
            <img class="rounded-lg w-full max-h-80 object-cover" src={project.cover_url} />
            <div class="h-10" />
            <div class="flex gap-8 relative flex-col md:flex-row">

                <div class="rounded p-4 shadow-accent shadow top-8 md:sticky h-fit text-left flex-shrink-0 flex flex-col gap-4">

                    {
                        if project.play_store_url.is_some() || project.web_url.is_some() {
                            Either::Left(
                                view! {
                                    <div>
                                        <div class="font-light text-sm text-accent"> "Available On" </div>
                                        <div class="flex gap-4">
                                            {
                                                if let Some(play_store) = project.play_store_url {
                                                    Either::Left(view! {
                                                        <a href=play_store class="font-semibold flex gap-2">
                                                            <Icon icon=crate::icons::Icons::GooglePlay />

                                                            "Play Store"
                                                        </a>
                                                    })
                                                }else{
                                                    Either::Right(())
                                                }
                                            }
                                            {
                                                if let Some(web_url) = project.web_url {
                                                    Either::Left(view! {
                                                        <a href=web_url class="font-semibold flex gap-2">
                                                            <Icon icon=crate::icons::Icons::Web />

                                                            "Web"
                                                        </a>
                                                    })
                                                }else{
                                                    Either::Right(())
                                                }
                                            }
                                        </div>
                                    </div>
                                }
                            )
                        }else{
                            Either::Right(())
                        }
                    }
                    {
                        if project.backend_source.is_some() || project.frontend_source.is_some() {
                            Either::Left(
                                view! {
                                    <div>
                                        <div class="font-light text-sm text-slate-300"> "Source" </div>
                                        <div class="flex gap-4">
                                            {
                                                if let Some(source) = project.frontend_source.clone() {
                                                    Either::Left(view! {
                                                        <a href=source class="font-semibold flex gap-2">
                                                            <Icon icon=crate::icons::Icons::Github />

                                                            {
                                                                if project.backend_source.is_none() {
                                                                    "Github"
                                                                }else {
                                                                    "Frontend"
                                                                }
                                                            }
                                                        </a>
                                                    })
                                                }else{
                                                    Either::Right(())
                                                }
                                            }
                                            {
                                                if let Some(source) = project.backend_source {
                                                    Either::Left(view! {
                                                        <a href=source class="font-semibold flex gap-2">
                                                            <Icon icon=crate::icons::Icons::Github />

                                                            {
                                                                if project.frontend_source.is_none() {
                                                                    "Github"
                                                                }else {
                                                                    "Backend"
                                                                }
                                                            }
                                                        </a>
                                                    })
                                                }else{
                                                    Either::Right(())
                                                }
                                            }
                                        </div>
                                    </div>
                                }
                            )
                        }else{
                            Either::Right(())
                        }
                    }
                    {
                        if let Some(stack) = project.stack {
                            Either::Left(view! {
                                <div>
                                    <div class="font-light text-sm text-slate-300"> "Stack" </div>
                                    <div class="font-semibold text-base"> {stack} </div>
                                </div>
                            })
                        }else{
                            Either::Right(())
                        }
                    }
                </div>

                <div class="text-left text-slate-300 project-md" inner_html={project.html} />
            </div>

            {
                if !project.screenshots.is_empty() {
                    Either::Left(view! {
                        <div class="h-10" />
                        <h2 class="text-5xl md:text-6xl font-bold text-left"> "Screenshots" <span class="text-accent"> "." </span> </h2>

                        <div class="h-4" />
                        <div class="flex gap-4 h-80 overflow-auto">
                            {
                                project.screenshots.into_iter().map(|sc_url|view! {
                                    <img class="h-full rounded" src=sc_url />
                                }).collect_view()
                            }
                        </div>
                    })
                }else{
                    Either::Right(())
                }
            }
            <div class="h-8" />
        </div>

        <div class="h-20" />
        <Footer />
    }
}
