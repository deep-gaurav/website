use leptos::{prelude::*, text_prop::TextProp};

#[component]
pub fn Picture(#[prop(into)] src: TextProp) -> impl IntoView {
    let src = src.get().as_str().to_string();
    let srcset = Resource::new_blocking(
        move || src.clone(),
        |src| async move {
            #[cfg(feature = "ssr")]
            {
                return ssr::make_variants(&src).await;
            }
            #[cfg(not(feature = "ssr"))]
            {
                return Option::<(String, String, (u32, u32))>::None;
            }
        },
    );
    Suspend::new(async move {
        let (srcset, sizes, width, height) = srcset
            .await
            .map(|(srcset, sizes, dim)| (Some(srcset), Some(sizes), Some(dim.0), Some(dim.1)))
            .unwrap_or((None, None, None, None));
        view! {
            <img
                src
                srcset={srcset}
                sizes={sizes}
                height={height}
                width={width}
            />
        }
    })
}

#[cfg(feature = "ssr")]
mod ssr {
    use std::path::PathBuf;

    use image::{imageops::FilterType::Lanczos3, ImageReader};
    use leptos::{config::LeptosOptions, prelude::expect_context};

    pub async fn make_variants(url: &str) -> Option<(String, String, (u32, u32))> {
        let options = expect_context::<LeptosOptions>();
        let path = PathBuf::from(&options.site_root).join(url.strip_prefix("/")?);
        let name = if let Some(extension) = path.extension() {
            path.file_name()?
                .to_str()?
                .strip_suffix(&format!(".{}", extension.to_str()?))?
        } else {
            path.file_name()?.to_str()?
        };
        let dir = path.parent()?;
        let image = ImageReader::open(&path).ok()?.decode().ok()?;
        let width = image.width();
        let height = image.height();
        let mut sizes = vec![480, 960, 1920];
        sizes.retain(|size| size < &width);

        if width > sizes.last().cloned().unwrap_or_default() {
            sizes.push(width);
        }

        let mut avif_sizes = vec![];
        for size in sizes.iter() {
            let img = image.clone();

            let new_h = ((*size as f64) / (width as f64)) * (height as f64);
            let new_img = img.resize_exact(*size, new_h as u32, Lanczos3);

            let name = format!("{name}-{size}.avif");
            let path = dir.join(name);

            if Some(false) == tokio::fs::try_exists(&path).await.ok() {
                if let Ok(_) = new_img.save_with_format(&path, image::ImageFormat::Avif) {
                    avif_sizes.push((*size, path));
                };
            }
        }
        avif_sizes.sort_by(|a, b| a.0.cmp(&b.0));
        let srcs = avif_sizes
            .iter()
            .map(|(k, v)| {
                format!(
                    "{} {k}w",
                    v.to_string_lossy()
                        .strip_prefix(&options.site_root)
                        .expect("expected path to start in root")
                )
            })
            .collect::<Vec<_>>()
            .join(", ");

        let sizes_st = avif_sizes
            .iter()
            .map(|(w, _)| {
                if w == &avif_sizes.last().map(|(w, _)| *w).unwrap_or_default() {
                    format!("{w}px")
                } else {
                    format!("(max-width: {w}px) {w}px")
                }
            })
            .collect::<Vec<_>>()
            .join(", ");

        Some((srcs, sizes_st, (width, height)))
    }
}
