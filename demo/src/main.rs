#![allow(non_snake_case)]

use base64::Engine;
use dioxus::prelude::*;

fn generate_image_url(src: &str) -> Option<String> {
    let image_data = rustmath::encode_png(src, true)?;

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
    let image_url = (*src.current())
        .as_ref()
        .and_then(|src| generate_image_url(src));

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
