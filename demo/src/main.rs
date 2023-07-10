#![allow(non_snake_case)]

use base64::Engine;
use dioxus::prelude::*;

mod uploader;

fn generate_image_url(src: &str, include_metadata: bool, png: bool) -> Option<String> {
    let (prefix, encoded) = if png {
        let image_data = rustmath::encode_png(src, include_metadata)?;
        let prefix = "data:image/png;base64,";
        let encoded = base64::engine::general_purpose::STANDARD.encode(&image_data);

        (prefix, encoded)
    } else {
        let image_data = rustmath::render_svg(src, include_metadata)?;
        let prefix = "data:image/svg+xml;base64,";
        let encoded = base64::engine::general_purpose::STANDARD.encode(&image_data);

        (prefix, encoded)
    };

    Some(prefix.to_string() + &encoded)
}

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    let src = use_state(cx, || None::<String>);
    let include_metadata = use_state(cx, || true);
    let render_png = use_state(cx, || true);

    let uploader = uploader::use_uploader(cx, src);

    let image_url = (*src.current()).as_ref().and_then(|src| {
        generate_image_url(src, *include_metadata.current(), *render_png.current())
    });

    let input_value = match src.as_ref() {
        Some(src) => src,
        None => "",
    };

    cx.render(rsx! {
        div {
            id: "rm-dropzone",
            onmounted: move |cx| { uploader.mount(&cx.inner()); },
        }
        div {
            id: "rm-app-container",
            section {
                class: "app",
                h1 {
                    "Demo"
                }
                input {
                    "type": "text",
                    value: input_value,
                    oninput: move |event| {
                        if event.data.value.is_empty() {
                            src.set(None);
                        } else {
                            src.set(Some(event.data.value.clone()));
                        }
                    },
                    placeholder: "Enter math code here"
                }
            }
            div {
                input {
                    "type": "checkbox",
                    "checked": "{include_metadata}",
                    onclick: move |_| include_metadata.set(!include_metadata),
                    id: "include_metadata"
                }
                label {
                    "for": "include_metadata",
                    "Include source as metadata"
                }
            }
            div {
                input {
                    "type": "checkbox",
                    "checked": "{render_png}",
                    onclick: move |_| render_png.set(!render_png),
                    id: "render_png"
                }
                label {
                    "for": "render_png",
                    "Render to PNG instead of SVG"
                }
            }
            div {
                class: "output",
                if let Some(url) = &image_url {
                    rsx! {
                        img {
                            src: "{url}"
                        }
                    }
                }
            }
        }
    })
}
