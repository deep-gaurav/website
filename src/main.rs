
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, generate_route_list_with_ssg, LeptosRoutes};
    use website::app::*;

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let (routes, static_routes) = generate_route_list_with_ssg({
        let leptos_options = leptos_options.clone();
        move || shell(leptos_options.clone())
    });

    static_routes.generate(&leptos_options).await;
    // tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}

#[cfg(all(not(feature="ssr"), feature="csr"))]
pub fn main() {
    use leptos::mount::mount_to_body;
    use website::app::App;
    console_error_panic_hook::set_once();
    mount_to_body(App)
}

#[cfg(all(not(feature="ssr"), not(feature="csr")))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
