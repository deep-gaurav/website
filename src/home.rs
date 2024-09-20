use leptos::{either::Either, prelude::*};

use crate::{
    footer::Footer,
    header::Header,
    project::{Project, PROJECTS},
    utils::Pairs,
};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="min-h-dvh w-full flex flex-col px-20">
            <Header />

            <div class="flex-grow w-full flex  justify-stretch items-center">
                <div class="flex flex-col">
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
                    <h2 class="text-left text-xl text-slate-300" > "I'm a software developer from India."<br/>"I love solving problems that bring people a little closer." </h2>
                    <div class="h-6" />
                    <div class="flex gap-4">
                        <button class="p-2 px-4 transition-colors bg-accent hover:bg-accent-dark rounded-md text-black font-medium"> "Get In Touch" </button>
                        <button class="p-2 px-4 border transition-colors border-white hover:bg-white hover:text-black rounded-md font-medium box-border"> "Browse Projects" </button>
                    </div>
                </div>

                <div class="flex-grow" />
                <div class="p-10 border border-solid border-accent rounded-full w-[30%]">
                    <div class="rounded-full relative overflow-hidden w-full aspect-square">
                        <img class="inline h-full w-full object-cover" src="/assets/images/deep.webp" />
                    </div>
                </div>
            </div>
        </div>
        <div class="px-20 flex flex-col">
            <h2 class="text-5xl font-semibold text-left"> "Projects" <span class="text-accent"> "." </span> </h2>
            <div class="h-6" />

            <div class="flex flex-col gap-10">
            {
                let project_pairs = Pairs::new( &PROJECTS[..3.min(PROJECTS.len())]);

                project_pairs.into_iter().enumerate().map(|(index, (project1, project2))|{
                    let (mut style1, mut style2) = ("flex-basis:40%; flex-grow:4;", "flex-basis:60%; flex-grow:6;");
                    if index %2== 0 {
                        (style1, style2) = (style2, style1);
                    }
                    view! {
                        <div class="flex gap-10">
                            <div
                                style=style1
                            >
                                <ProjectCard project=project1 />
                            </div>
                            {
                                if let Some(project2) = project2 {
                                    Either::Left(view! {
                                        <div
                                            style=style2
                                        >
                                            <ProjectCard project=project2 />
                                        </div>
                                    })
                                }else{
                                    Either::Right(())
                                }
                            }
                        </div>
                    }
                }).collect_view()
            }
            </div>
            <div class="h-8" />

            <div class="text-2xl text-left text-slate-200 font-medium"> "Interested in more projects" <span class="text-accent"> "?" </span> </div>
            <div class="h-2" />
            <button class="self-start p-2 px-4 transition-colors bg-accent hover:bg-accent-dark rounded-md text-black font-medium"> "View All Projects" </button>

        </div>

        <div class="h-20" />
        <div class="px-20 flex flex-col">
            <h2 class="text-5xl font-semibold text-left"> "My Story" <span class="text-accent"> "." </span> </h2>
            <div class="h-6" />

            <p class="text-left text-slate-300 text-lg whitespace-pre-line">
                r#"From a young age, I was drawn to computers, experimenting with technology even before we had a TV at home. I started building games with no-code engines, eventually learning to code. After pursuing a degree in computer science at Delhi University, I quickly picked up new technologies like Flutter and developed my first app, MusicPiped, which gained traction in the open-source community.

During the pandemic, I developed multiplayer games to connect friends online. This experience, coupled with my role as a frontend developer at Akudo, a fintech organization, deepened my technical expertise. I also ventured into Rust programming, finding joy in its capabilities.

My passion for creating connective technologies continued with apps like SyncPlayer (later TVMate) for shared video experiences. Currently, I'm applying my skills at Mitsu.care, developing solutions for assisted self-therapy.

Throughout my career, I've remained committed to crafting solutions that bring people closer together, one app at a time."#
            </p>
        </div>

        <div class="h-20" />

        <Footer />
    }
}

#[component]
fn ProjectCard<'a>(project: &'a Project) -> impl IntoView + use<'a> {
    view! {
        <img class="rounded-xl h-80 w-full object-cover" src=project.cover_image.as_ref() />
        <div class="h-4" />
        <h3 class="text-left text-3xl font-semibold"> {project.name.as_ref()} </h3>
        <div class="h-2" />
        <p class="text-left text-slate-300 text-lg"> {project.short_description.as_ref()} </p>
    }
}
