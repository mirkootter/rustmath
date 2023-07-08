use dioxus::{core::ScopeState, prelude::MountedData};
use wasm_bindgen::{prelude::Closure, JsCast};

#[derive(Default)]
pub struct UseUploader {
    _inner: Option<Inner>,
}

impl UseUploader {
    pub fn mount(&mut self, data: &MountedData) {
        assert!(self._inner.is_none());

        let element = data.get_raw_element().unwrap();
        let element = element.downcast_ref::<web_sys::Element>().unwrap().clone();

        self._inner = Some(Inner::new(element));
    }
}

struct Inner {
    element: web_sys::Element,
    handler: Closure<dyn Fn()>,
}

impl Inner {
    pub fn new(element: web_sys::Element) -> Self {
        let window = web_sys::window().unwrap();

        let handler = Closure::new(move || {
            let _ = window.alert_with_message("Clicked");
        });

        element
            .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
            .unwrap();

        Self { element, handler }
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        let _ = self
            .element
            .remove_event_listener_with_callback("click", self.handler.as_ref().unchecked_ref());
    }
}

pub fn use_uploader<'a>(cx: &'a ScopeState) -> &'a mut UseUploader {
    cx.use_hook(|| UseUploader::default())
}
