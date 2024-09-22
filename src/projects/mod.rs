use futures::{stream::FuturesUnordered, TryStreamExt};
use leptos::either::Either;
use leptos::prelude::*;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;
use markdown::{mdast::Node, Constructs, Options, ParseOptions};
use project_view::ProjectView;
use serde::{Deserialize, Serialize};
use yaml_rust2::{Yaml, YamlLoader};

pub mod project_view;

#[derive(Serialize, Deserialize, Clone)]
pub struct ProjectData {
    pub priority: i64,
    pub title: String,
    pub tagline: String,
    pub cover_url: String,
    pub short_description: String,
    pub slug: String,
    pub stack: Option<String>,
    pub web_url: Option<String>,
    pub play_store_url: Option<String>,
    pub backend_source: Option<String>,
    pub frontend_source: Option<String>,
    pub screenshots: Vec<String>,

    pub html: String,
}

impl ProjectData {
    pub fn url(&self) -> String {
        format!("/projects/{}/", self.slug)
    }
}

#[server]
pub async fn list_project_slugs() -> Result<Vec<String>, ServerFnError> {
    use tokio::fs;
    use tokio_stream::wrappers::ReadDirStream;
    use tokio_stream::StreamExt;

    let files = ReadDirStream::new(fs::read_dir("./projects").await?);
    Ok(files
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if !path.is_file() {
                return None;
            }
            let extension = path.extension()?;
            if extension != "md" {
                return None;
            }

            let slug = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default()
                .replace(".md", "");
            Some(slug)
        })
        .collect()
        .await)
}

#[server]
pub async fn list_projects() -> Result<Vec<ProjectData>, ServerFnError> {
    let slugs = list_project_slugs().await?;
    let projects_fut = slugs
        .into_iter()
        .map(|slug| get_project(slug))
        .collect::<FuturesUnordered<_>>();
    let mut projects: Vec<ProjectData> = projects_fut.try_collect().await?;
    projects.sort_by(|a, b| b.priority.cmp(&a.priority));
    Ok(projects)
}

#[server]
pub async fn get_project(slug: String) -> Result<ProjectData, ServerFnError> {
    let content = tokio::fs::read_to_string(&format!("./projects/{slug}.md")).await?;
    let md = markdown::to_mdast(
        &content,
        &ParseOptions {
            constructs: Constructs {
                frontmatter: true,
                ..Constructs::default()
            },
            ..ParseOptions::default()
        },
    )
    .map_err(|_| ServerFnError::new("Cannot parse md {e:?}"))?;

    let yaml = md
        .children()
        .map(|c| c.iter().find(|p| matches!(p, Node::Yaml(..))))
        .flatten()
        .ok_or(ServerFnError::new("cant find yaml"))?;
    if let Node::Yaml(yaml) = yaml {
        let docs = YamlLoader::load_from_str(&yaml.value)
            .map_err(|_| ServerFnError::new("yaml parse failed"))?;
        let doc = docs
            .get(0)
            .and_then(|d| d.as_hash())
            .ok_or(ServerFnError::new("No doc in yaml or not hash"))?;
        let title = doc
            .get(&Yaml::from_str("title"))
            .and_then(|t| t.as_str())
            .ok_or(ServerFnError::new("title not str"))?;
        let short_description = doc
            .get(&Yaml::from_str("short_description"))
            .and_then(|t| t.as_str())
            .ok_or(ServerFnError::new("no short short_description"))?;

        let tagline = doc
            .get(&Yaml::from_str("tagline"))
            .and_then(|t| t.as_str())
            .ok_or(ServerFnError::new("no short tagline"))?;
        let cover_url = doc
            .get(&Yaml::from_str("cover"))
            .and_then(|t| t.as_str())
            .ok_or(ServerFnError::new("no cover in yaml"))?;

        let stack = doc
            .get(&Yaml::from_str("stack"))
            .map(|t| t.as_str())
            .flatten()
            .map(|t| t.to_string());
        let web_url = doc
            .get(&Yaml::from_str("web_url"))
            .map(|t| t.as_str())
            .flatten()
            .map(|t| t.to_string());
        let play_store_url = doc
            .get(&Yaml::from_str("play_store_url"))
            .map(|t| t.as_str())
            .flatten()
            .map(|t| t.to_string());
        let backend_source = doc
            .get(&Yaml::from_str("backend_source"))
            .map(|t| t.as_str())
            .flatten()
            .map(|t| t.to_string());
        let frontend_source = doc
            .get(&Yaml::from_str("frontend_source"))
            .map(|t| t.as_str())
            .flatten()
            .map(|t| t.to_string());
        let screenshots = doc
            .get(&Yaml::from_str("screenshots"))
            .map(|t| t.as_vec())
            .flatten()
            .map(|t| {
                t.clone()
                    .into_iter()
                    .filter_map(|t| t.as_str().map(|t| t.to_string()))
                    .collect::<Vec<_>>()
            });

        let priority = doc
            .get(&Yaml::from_str("priority"))
            .map(|t| t.as_i64())
            .flatten()
            .unwrap_or_default();

        let html = markdown::to_html_with_options(
            &content,
            &Options {
                parse: ParseOptions {
                    constructs: Constructs {
                        frontmatter: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
        )
        .map_err(|_| ServerFnError::new("Error converting md to html"))?;
        Ok(ProjectData {
            title: title.to_string(),
            cover_url: cover_url.to_string(),
            short_description: short_description.to_string(),
            slug: slug,
            tagline: tagline.to_string(),
            screenshots: screenshots.unwrap_or_default(),
            html,

            stack,
            web_url,
            play_store_url,
            backend_source,
            frontend_source,
            priority,
        })
    } else {
        Err(ServerFnError::new("node not yaml"))
    }
}

#[derive(Params, Clone, Debug, PartialEq, Eq)]
pub struct ProjectParams {
    slug: Option<String>,
}

#[component]
pub fn ProjectPage() -> impl IntoView {
    let query = use_params::<ProjectParams>();
    let slug = move || query.get().map(|q| q.slug.unwrap_or_default());
    let project_resource = Resource::new_blocking(slug, |slug| async move {
        match slug {
            Err(e) => Err(format!("{e:#?}")),
            Ok(slug) => get_project(slug).await.map_err(|e| format!("{e:#?}")),
        }
    });
    let post_view = move || {
        Suspend::new(async move {
            match project_resource.await {
                Ok(project) => Either::Left({
                    view! {<ProjectView project />}
                }),
                Err(e) => Either::Right(view! {{e}}),
            }
        })
    };

    view! {
        <Suspense fallback=move || view! { <p>"Loading post..."</p> }>
            <ErrorBoundary fallback=|errors| {
                #[cfg(feature = "ssr")]
                expect_context::<leptos_axum::ResponseOptions>()
                    .set_status(http::StatusCode::NOT_FOUND);
                view! {
                    <div class="error">
                        <h1>"Something went wrong."</h1>
                        <ul>

                            {move || {
                                errors
                                    .get()
                                    .into_iter()
                                    .map(|(_, error)| view! { <li>{error.to_string()}</li> })
                                    .collect::<Vec<_>>()
                            }}

                        </ul>
                    </div>
                }
            }>{post_view}</ErrorBoundary>
        </Suspense>
    }
}
