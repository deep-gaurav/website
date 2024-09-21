use std::borrow::Cow;

use leptos::{either::Either, prelude::*};

use crate::{footer::Footer, header::Header, utils::Pairs};

pub struct Project {
    pub name: Cow<'static, str>,
    pub cover_image: Cow<'static, str>,
    pub short_description: Cow<'static, str>,
}

pub const PROJECTS: &[Project] = &[
    Project {
        name: std::borrow::Cow::Borrowed("BillDivide"),
        cover_image: std::borrow::Cow::Borrowed("/assets/images/projects/billdivide/cover.png"),
        short_description: std::borrow::Cow::Borrowed(
            r#"BillDivide is a simple and easy-to-use app that lets you split bills with friends, family, or anyone else. Whether you're dining out, going on vacation, or just sharing household expenses, BillDivide makes it easy to track who owes what and settle up quickly and easily."#,
        ),
    },
    Project {
        name: std::borrow::Cow::Borrowed("TVMate"),
        cover_image: std::borrow::Cow::Borrowed("/assets/images/projects/tvmate/cover.png"),
        short_description: std::borrow::Cow::Borrowed(
            r#"A nostalgic video-syncing app with CRT TV aesthetics, allowing users to watch movies together remotely while chatting in a shared virtual room."#,
        ),
    },
    Project {
        name: std::borrow::Cow::Borrowed("SelfCloud"),
        cover_image: std::borrow::Cow::Borrowed("/assets/images/projects/selfcloud/cover.png"),
        short_description: std::borrow::Cow::Borrowed(
            r#"A self-hosted platform that simplifies project deployment by managing containers, domains, and SSL certificates, allowing easy routing of traffic to specific project ports through a reverse proxy system."#,
        ),
    },
    Project {
        name: std::borrow::Cow::Borrowed("MusicPiped"),
        cover_image: std::borrow::Cow::Borrowed("/assets/images/projects/musicpiped/cover.png"),
        short_description: std::borrow::Cow::Borrowed(
            r#"MusicPiped is a material-designed music player that streams audio from YouTube, allowing ad-free background playback and offline caching. It offers a unique blend of YouTube Music features and traditional music player functionality without relying on Google APIs."#,
        ),
    },
    Project {
        name: std::borrow::Cow::Borrowed("BingoTingo"),
        cover_image: std::borrow::Cow::Borrowed("/assets/images/projects/bingotingo/cover.png"),
        short_description: std::borrow::Cow::Borrowed(
            r#"A real-time online gaming web app merging Bingo and Lines/Boxes for endless fun and social interaction."#,
        ),
    },
];

#[component]
pub fn ProjectCard<'a>(project: &'a Project) -> impl IntoView + use<'a> {
    view! {
        <img class="rounded-xl h-80 w-full object-cover bg-white/10" src=project.cover_image.as_ref() />
        <div class="h-4" />
        <h3 class="text-left text-3xl font-semibold"> {project.name.as_ref()} </h3>
        <div class="h-2" />
        <p class="text-left text-slate-300 text-lg"> {project.short_description.as_ref()} </p>


    }
}

#[component]
pub fn ProjectsPage() -> impl IntoView {
    view! {
        <Header />
        <div class="min-h-dvh w-full flex flex-col px-8 md:px-20">
            <div class="h-10 md:h-20" />
            <h1 class="text-5xl md:text-7xl font-bold text-left"> "My "<span class="text-accent">"Digital"</span>" Creations" </h1>
            <div class="h-8" />
            <h2 class="text-left text-xl text-slate-300" > "From social apps to cloud solutions: a showcase of innovative projects solving real-world problems" </h2>

            <div class="h-20" />
            <div class="flex flex-col gap-10">
                {
                    let project_pairs = Pairs::new(PROJECTS);

                    view! {
                        <ProjectList project_pairs />
                    }
                }
            </div>
        </div>

        <div class="h-20" />
        <Footer />
    }
}

#[component]
pub fn ProjectList<'a>(project_pairs: Pairs<'a, Project>) -> impl IntoView + use<'a> {
    project_pairs
        .into_iter()
        .enumerate()
        .map(|(index, (project1, project2))| {
            let (mut style1, mut style2) = (
                "flex-basis:40%; flex-grow:4;",
                "flex-basis:60%; flex-grow:6;",
            );
            if index % 2 == 0 {
                (style1, style2) = (style2, style1);
            }
            view! {
                <div class="flex gap-10 flex-col md:flex-row">
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
        })
        .collect_view()
}
