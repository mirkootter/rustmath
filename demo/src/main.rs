#![allow(non_snake_case)]

use base64::Engine;
use dioxus::prelude::*;

mod uploader;

fn generate_image_url(src: &str, include_metadata: bool) -> Option<String> {
    let image_data = rustmath::encode_png(src, include_metadata)?;

    let prefix = "data:image/png;base64,".to_string();
    let encoded = base64::engine::general_purpose::STANDARD.encode(&image_data);
    Some(prefix + &encoded)
}

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    let src = use_state(cx, || None::<String>);
    let include_metadata = use_state(cx, || true);

    let _uploader = uploader::use_uploader(cx);

    let image_url = (*src.current())
        .as_ref()
        .and_then(|src| generate_image_url(src, *include_metadata.current()));

    cx.render(rsx! {
        section {
            class: "app",
            h1 {
                "Demo"
            }
            input {
                "type": "text",
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
            class: "output",
            if let Some(url) = &image_url {
                rsx! {
                    img {
                        src: "{url}"
                    }
                }
            }
        }
    })
}
