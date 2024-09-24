use leptos::prelude::*;
use leptos_router::components::A;

use crate::{
    about::MyStory,
    footer::Footer,
    header::{Header, MenuPage},
    picture::Picture,
    project::ProjectList,
    projects::list_projects,
    utils::{title::SiteMeta, Pairs},
};

#[component]
pub fn HomePage() -> impl IntoView {
    let projects_resource = Resource::new_blocking(
        || (),
        |_| async move { list_projects().await.unwrap_or_default() },
    );
    view! {
        <SiteMeta title="Home" description="I'm a software developer from India. I love solving problems that bring people a little closer." />
        <Header />

        <div id="og-image" class="min-h-svh w-full flex flex-col px-8 md:px-20">
            <div class="flex-grow w-full flex flex-col md:flex-row justify-stretch items-center">
                <div class="flex-grow md:flex-grow-0" />
                <div class="flex flex-col">
                    <div class="text-slate-300 text-lg text-left">
                        "Hi, I'm Deep 👋"
                    </div>
                    <div class="h-6" />
                    <h1 class="text-5xl md:text-7xl font-bold text-left ">
                        <span class="text-accent"> "Software" </span>
                        <br/>
                        "Developer"
                    </h1>
                    <div class="h-8" />
                    <h2 class="text-left text-lg md:text-xl text-slate-300" > "I'm a software developer from India."<br/>"I love solving problems that bring people a little closer." </h2>
                    <div class="h-6" />
                    <div class="flex gap-4">
                        <A attr:aria-label={MenuPage::Contact.aria_label()} href={MenuPage::Contact.path()} {..view! {< {..} class="p-2 px-4 transition-colors bg-accent hover:bg-accent-dark rounded-md text-black font-medium" />}} > "Get In Touch" </A>
                        <A attr:aria-label={MenuPage::Projects.aria_label()} href={MenuPage::Projects.path()} {..view! {< {..} class="p-2 px-4 border transition-colors border-white hover:bg-white hover:text-black rounded-md font-medium box-border" />}}> "Browse Projects" </A>
                    </div>
                </div>

                <div class="flex-grow min-h-2 flex-shrink-0" />
                <div class="p-8 md:p-10 border border-solid border-accent rounded-full w-full md:w-[30%]">
                    <div class="rounded-full relative overflow-hidden w-full aspect-square">
                        <Picture src="/assets/images/deep.webp"
                            attr:class="inline h-full w-full object-cover"
                            alt="Deep Gaurav, Profile Picture"
                            sizes="(max-width: 768px) 100vw, (max-width: 1024px) 50vw, 33vw"
                        />
                    </div>
                </div>
                <div class="flex-grow md:flex-grow-0" />
            </div>
        </div>
        <div class="px-8 md:px-20 flex flex-col">
            <h2 class="text-5xl font-semibold text-left"> "Projects" <span class="text-accent"> "." </span> </h2>
            <div class="h-6" />

            <div class="flex flex-col gap-10">
            {
                Suspend::new(
                    async move {
                        let projects = projects_resource.await;
                        let project_pairs = Pairs::new(projects.into_iter().take(3));

                        view! {
                            <ProjectList project_pairs
                                is_first_lazy=true
                                is_rest_lazy=true
                            />
                        }

                    }
                )
            }
            </div>
            <div class="h-8" />

            <div class="text-2xl text-left text-slate-200 font-medium"> "Interested in more projects" <span class="text-accent"> "?" </span> </div>
            <div class="h-2" />
            <A attr:aria-label={MenuPage::Projects.aria_label()} href={MenuPage::Projects.path()} {..view! {< {..} class="self-start p-2 px-4 transition-colors bg-accent hover:bg-accent-dark rounded-md text-black font-medium" />}}> "View All Projects" </A>

        </div>

        <div class="h-20" />

        <MyStory />

        <div class="h-20" />

        <Footer />
    }
}
