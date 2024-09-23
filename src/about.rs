use std::borrow::Cow;

use leptos::prelude::*;
use leptos_meta::Script;

use crate::{footer::Footer, header::Header, icons::Icon, utils::title::SiteMeta};

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <SiteMeta title="About" description="Tech enthusiast building connective solutions, one app at a time. Passionate about crafting innovative experiences that bring people closer together." />
        <Header />

        <div id="og-image" class="flex flex-col px-8 md:px-20">

            <div class="h-20" />
            <h1 class="text-5xl md:text-7xl font-bold text-left"> "About me"<span class="text-accent">"."</span></h1>
            <div class="h-8" />
            <h2 class="text-left text-xl text-slate-300" > "Tech enthusiast building connective solutions, one app at a time. Passionate about crafting innovative experiences that bring people closer together." </h2>
            <div class="h-10" />
            <div class="flex gap-10 items-stretch flex-col md:flex-row">
                <div class="flex flex-col flex-shrink-0 flex-grow-[1]  basis-0 relative w-full md:w-1">
                    <h3 class="text-2xl font-semibold text-left"> "My Stack" </h3>
                    <div class="h-4" />
                    <TechStack />
                </div>
                <div class="flex flex-col flex-shrink-0 flex-grow-[2] basis-0 relative w-full md:w-1">
                    <h3 class="text-2xl font-semibold text-left"> "My Place" </h3>
                    <div class="h-4" />
                    <Location />
                </div>
            </div>
        </div>

        <div class="h-20" />
        <MyStory />

        <div class="h-20" />
        <MyExperience />

        <div class="h-20" />

        <Footer />
    }
}

#[component]
fn TechStack() -> impl IntoView {
    let frontend_stack: &[&'static str] = &[
        "HTML/CSS",
        "Leptos (Rust)",
        "Flutter",
        "Tailwind",
        "ReactJS",
        "Android (Kotlin/Java)",
        "iOS (Swift)",
    ];

    let backend_stack: &[&'static str] = &[
        "Axum (Rust)",
        "Leptos (Rust)",
        "GraphQL",
        "REST API",
        "WebSocket",
        "WebRTC",
        "WebTransport",
    ];

    let other_stack: &[&'static str] = &[
        "SQLite",
        "Postgres",
        "Docker",
        "Podman",
        "Litestream",
        "WebTransport",
    ];

    let stacks = [frontend_stack, backend_stack, other_stack];

    view! {
        <div class="bg-white/10 py-20 flex flex-col justify-center overflow-hidden marquee rounded-md gap-4 flex-grow">
            {
                stacks.iter().enumerate().map(|(i,stack)|{
                    view! {
                        <div class=format!("flex marquee-content gap-4 {}", if i%2==0 {"marquee-forward"}else {"marquee-rev"})>
                            {
                                stack.iter().chain(stack.iter()).map(
                                    |tech| view! {
                                        <span class="bg-slate-300 text-black p-2 rounded text-nowrap"> {*tech} </span>
                                        <Icon {..view! {< {..} class="text-accent" />}} icon=crate::icons::Icons::Star />
                                    }
                                ).collect_view()
                            }
                        </div>
                    }
                }).collect_view()
            }
        </div>
    }
}

#[component]
fn Location() -> impl IntoView {
    view! {
        <div class="bg-white/10 overflow-hidden rounded-md flex justify-center items-center flex-col md:flex-row p-2">
            <div class="aspect-square w-full max-w-[400px]">
                <canvas id="cobe" class="w-full h-full cursor-grab" />
                <Script defer="true" type_="module" src="/assets/scripts/cobe.js" />
            </div>
            <div class="text-slate-200 text-lg flex gap-4 items-center">
                <div class="w-4 h-4 bg-accent rounded-full" />
                "Delhi NCR, India"
            </div>
        </div>
    }
}

#[component]
pub fn MyStory() -> impl IntoView {
    view! {
        <div class="px-8 md:px-20 flex flex-col">
            <h2 class="text-5xl font-semibold text-left"> "My Story" <span class="text-accent"> "." </span> </h2>
            <div class="h-6" />

            <p class="text-left text-slate-300 text-lg whitespace-pre-line">
                r#"From a young age, I was drawn to computers, experimenting with technology even before we had a TV at home. I started building games with no-code engines, eventually learning to code. After pursuing a degree in computer science at Delhi University, I quickly picked up new technologies like Flutter and developed my first app, MusicPiped, which gained traction in the open-source community.

    During the pandemic, I developed multiplayer games to connect friends online. This experience, coupled with my role as a frontend developer at Akudo, a fintech organization, deepened my technical expertise. I also ventured into Rust programming, finding joy in its capabilities.

    My passion for creating connective technologies continued with apps like SyncPlayer (later TVMate) for shared video experiences. Currently, I'm applying my skills at Mitsu.care, developing solutions for assisted self-therapy.

    Throughout my career, I've remained committed to crafting solutions that bring people closer together, one app at a time."#
            </p>
        </div>
    }
}

#[component]
pub fn MyExperience() -> impl IntoView {
    struct Experience {
        company: Cow<'static, str>,
        designation: Cow<'static, str>,
        content: Cow<'static, str>,
    }

    const EXPERIENCES: &[Experience] = &[
        Experience {
            company: Cow::Borrowed("Akudo"),
            designation: Cow::Borrowed("Software Engineer"),
            content: Cow::Borrowed(
                r#"During my time at Akudo Technologies, I led the development of a core fintech app built on Flutter, which achieved over 1.5 million downloads. I was instrumental in designing and implementing UPI on PPI, driving a 5x increase in transaction volume. Additionally, I developed an automated A/B testing system that empowered the product team to conduct real-time tests, leading to significant improvements in user experience. I also took on a mentorship role, guiding interns and junior developers, helping them grow in their professional journeys.

Earlier in my tenure as a Software Engineer, I contributed to the launch and maintenance of the company's flagship app from day one. I created a highly efficient build automation platform that reduced testing time and sped up the release cycle. I also engineered a custom Parallel BLoC system that reduced app startup time by 3x and integrated multiple payment gateways, helping to streamline the app's financial operations."#,
            ),
        },
        Experience {
            company: Cow::Borrowed("Mitsu"),
            designation: Cow::Borrowed("Frontend Developer"),
            content: Cow::Borrowed(
                r#"At Mitsu.care, I lead front-end development for a self-therapy app, aiming to support those with anxiety and depression. My goal is to create intuitive and seamless user experiences.

Since joining in August 2023, I've developed a web app version, significantly improving onboarding, and maintain user-facing interfaces across the app, web flow, website, and admin dashboard using Flutter, React JS, and Wordpress."#,
            ),
        },
    ];
    view! {
        <div class="px-8 md:px-20 flex flex-col">
            <h2 class="text-5xl font-semibold text-left"> "My Experience" <span class="text-accent"> "." </span> </h2>
            <div class="h-10" />

            {
                EXPERIENCES.iter().enumerate().map(|(index,experience)|{
                    view! {
                        <div>
                            <div class="flex flex-col md:flex-row">
                                <div class="flex gap-4">
                                    <div class="md:ml-4 text-5xl font-bold flex-shrink-0 md:w-32 text-left"> {format!("{:02}", index+1)}  </div>
                                    <div class="flex flex-col">
                                        <div class="text-accent md:hidden text-left"> {experience.designation.as_ref()} </div>
                                        <div class="font-bold text-2xl md:hidden text-left"> {experience.company.as_ref()} </div>
                                    </div>
                                </div>
                                <div class="h-2" />
                                <div class="text-left flex flex-col">
                                    <div class="text-accent hidden md:block"> {experience.designation.as_ref()} </div>
                                    <div class="font-bold text-2xl hidden md:block"> {experience.company.as_ref()} </div>
                                    <div class="text-slate-300 w-full whitespace-break-spaces"> {experience.content.as_ref()} </div>
                                </div>
                            </div>
                            <div class="h-12" />
                        </div>
                    }
                }).collect_view()
            }
        </div>
    }
}
