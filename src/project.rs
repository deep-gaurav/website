use leptos::{either::Either, prelude::*};

use crate::{
    footer::Footer,
    header::Header,
    icons::Icon,
    picture::Picture,
    projects::{list_projects, ProjectData},
    utils::{title::SiteMeta, Pairs},
};

#[component]
pub fn ProjectCard(project: ProjectData, #[prop(optional)] lazy: bool) -> impl IntoView {
    let url = project.url();
    view! {
        <a aria-label=format!("View Project {}", project.title) href=url class="flex flex-col hover:bg-white/10 transition-all duration-300">
            <Picture attr:class="rounded-xl max-h-80 w-full object-cover bg-white/10" src=project.cover_url alt=format!("{} cover image", project.title)
                attr:loading={
                    if lazy {
                        Some("lazy")
                    }else{
                        None
                    }
                }
            />
            <div class="h-4" />
            <div class="flex">
                <h3 class="text-left text-3xl font-semibold"> {project.title.clone()} </h3>
                <div class="flex-grow" />
                <Icon icon=crate::icons::Icons::Open />
            </div>
            <div class="h-2" />
            <p class="text-left text-slate-300 text-lg"> {project.tagline} </p>
        </a>
    }
}

#[component]
pub fn ProjectsPage() -> impl IntoView {
    let projects_resource = Resource::new_blocking(
        || (),
        |_| async move { list_projects().await.unwrap_or_default() },
    );

    view! {
        <SiteMeta title="Projects" description="From social apps to cloud solutions: a showcase of innovative projects solving real-world problems" />
        <Header />

        <div id="og-image" class=" w-full flex flex-col px-8 md:px-20">

            <div class="h-10 md:h-20" />
            <h1 class="text-5xl md:text-7xl font-bold text-left"> "My "<span class="text-accent">"Digital"</span>" Creations" </h1>
            <div class="h-8" />
            <h2 class="text-left text-xl text-slate-300" > "From social apps to cloud solutions: a showcase of innovative projects solving real-world problems" </h2>

        </div>
        <div class="w-full flex flex-col px-8 md:px-20">
            <div class="h-10" />
            <div class="flex flex-col gap-10">
                {
                    Suspend::new(
                        async move {
                            let projects = projects_resource.await;
                            let project_pairs = Pairs::new(projects.into_iter());

                            view! {
                                <ProjectList project_pairs
                                    is_first_lazy=false
                                    is_rest_lazy=true
                                 />
                            }

                        }
                    )
                }
            </div>
        </div>

        <div class="h-20" />
        <Footer />
    }
}

#[component]
pub fn ProjectList<I>(
    project_pairs: Pairs<I>,
    #[prop(optional)] is_staggered: Option<bool>,
    #[prop(optional)] is_first_lazy: bool,
    #[prop(optional)] is_rest_lazy: bool,
) -> impl IntoView
where
    I: Iterator<Item = ProjectData> + ExactSizeIterator,
{
    let is_staggered = is_staggered.unwrap_or(true);
    project_pairs
        .into_iter()
        .enumerate()
        .map(|(index, (project1, project2))| {
            let (mut style1, mut style2) = if is_staggered {
                (
                    "flex-basis:40%; flex-grow:4;",
                    "flex-basis:60%; flex-grow:6;",
                )
            } else {
                (
                    "flex-basis:50%; flex-grow:5;",
                    "flex-basis:50%; flex-grow:5;",
                )
            };
            if index % 2 == 0 {
                (style1, style2) = (style2, style1);
            }
            view! {
                <div class="flex gap-10 flex-col md:flex-row">
                    <div
                        style=style1
                    >
                        <ProjectCard project=project1
                            lazy={
                                if index ==0 {
                                    is_first_lazy
                                }else{
                                    is_rest_lazy
                                }
                            }
                        />
                    </div>
                    {
                        if let Some(project2) = project2 {
                            Either::Left(view! {
                                <div
                                    style=style2
                                >
                                    <ProjectCard project=project2 lazy={is_rest_lazy} />
                                </div>
                            })
                        }else{
                            Either::Right(())
                        }
                    }
                </div>
            }
        })
        .collect_view()
}
