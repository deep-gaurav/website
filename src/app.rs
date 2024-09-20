use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
    static_routes::StaticRoute,
    SsrMode, StaticSegment,
};

use crate::{header::Header, home::HomePage, project::ProjectsPage};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options islands=true/>
                <link rel="stylesheet" id="leptos" href="/pkg/leptos_tailwind.css"/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        {
            #[cfg(not(feature="csr"))]{
                view! {
                    <Stylesheet id="leptos" href="/pkg/website.css"/>
                }
            }
        }

        // sets the document title
        <Title text="Deep Gaurav"/>

        // content for this welcome page
        <Router>
            <main class="w-full h-full bg-black/90 text-white overflow-auto">
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage
                        ssr=SsrMode::Static(
                            StaticRoute::new(),
                        )
                    />
                    <Route path=path!("projects/") view=ProjectsPage
                        ssr=SsrMode::Static(
                            StaticRoute::new(),
                        )
                    />
                </Routes>
            </main>
        </Router>
    }
}
