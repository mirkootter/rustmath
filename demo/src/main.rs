#![allow(non_snake_case)]

use dioxus::prelude::*;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        section {
            class: "app",
            h1 {
                "Demo"
            }
            input {
                "type": "text",
                placeholder: "Enter math code here"
            }
        }
        div {
            class: "output",
            "Some output"
        }
    })
}
