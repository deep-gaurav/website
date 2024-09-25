#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use std::path::PathBuf;

    use leptos::{config::get_configuration, prelude::provide_context};
    use leptos_axum::generate_route_list_with_exclusions_and_ssg_and_context;
    use website::{app::*, picture::ssr::VariantLock};

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let lc = leptos_options.clone();
    let target_folder = PathBuf::from(&lc.site_root)
        .parent()
        .unwrap()
        .join("images");
    let variant_lock = VariantLock::new(target_folder);
    let _ = tokio::fs::create_dir_all(&variant_lock.cache_folder_path).await;
    println!("Generate routes");
    let (_, static_routes) = generate_route_list_with_exclusions_and_ssg_and_context(
        {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        },
        None,
        move || {
            provide_context(lc.clone());
            provide_context(variant_lock.clone());
        },
    );

    static_routes.generate(&leptos_options).await;
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    use leptos::mount::mount_to_body;
    use website::app::App;
    console_error_panic_hook::set_once();
    mount_to_body(App)
}

#[cfg(all(not(feature = "ssr"), not(feature = "csr")))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
