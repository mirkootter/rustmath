use dioxus::{core::ScopeState, prelude::MountedData};
use wasm_bindgen::{prelude::Closure, JsCast};
use wasm_bindgen_futures::JsFuture;

async fn read_uploaded_file(ev: web_sys::DragEvent) -> Option<Vec<u8>> {
    let data_transfer = ev.data_transfer()?;
    let files = data_transfer.files()?;

    if files.length() != 1 {
        return None;
    }

    let file = files.get(0)?;
    let array_buffer = JsFuture::from(file.array_buffer()).await.ok()?;
    let array_buffer = js_sys::ArrayBuffer::unchecked_from_js(array_buffer);

    let arr = js_sys::Uint8Array::new(&array_buffer);
    let len = arr.byte_length() as usize;

    let mut result = Vec::with_capacity(len);
    unsafe {
        result.set_len(len);
    }
    arr.copy_to(&mut result);

    Some(result)
}

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
    dragover: Closure<dyn Fn(web_sys::DragEvent)>,
    dragleave: Closure<dyn Fn(web_sys::Event)>,
    drop: Closure<dyn Fn(web_sys::DragEvent)>,
}

impl Inner {
    pub fn new(element: web_sys::Element) -> Self {
        let elem_owned = element.clone();
        let dragover = Closure::new(move |ev: web_sys::DragEvent| {
            ev.prevent_default();
            elem_owned.set_class_name("active");
        });

        let elem_owned = element.clone();
        let dragleave = Closure::new(move |_| {
            elem_owned.set_class_name("");
        });

        let elem_owned = element.clone();
        let drop = Closure::new(move |ev: web_sys::DragEvent| {
            ev.prevent_default();
            elem_owned.set_class_name("");

            let task = async {
                if let Some(data) = read_uploaded_file(ev).await {
                    if let Some(source) = rustmath::get_source_from_png_metadata(&data) {
                        let window = web_sys::window().unwrap();
                        let _ = window.alert_with_message(&source);
                        // TODO
                    } else {
                        let window = web_sys::window().unwrap();
                        let _ = window.alert_with_message("Unsupported file.");
                    }
                }
            };
            wasm_bindgen_futures::spawn_local(task);
        });

        element
            .add_event_listener_with_callback("dragover", dragover.as_ref().unchecked_ref())
            .unwrap();

        element
            .add_event_listener_with_callback("dragleave", dragleave.as_ref().unchecked_ref())
            .unwrap();

        element
            .add_event_listener_with_callback("drop", drop.as_ref().unchecked_ref())
            .unwrap();

        Self {
            element,
            dragover,
            dragleave,
            drop,
        }
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        let _ = self.element.remove_event_listener_with_callback(
            "dragover",
            self.dragover.as_ref().unchecked_ref(),
        );
        let _ = self.element.remove_event_listener_with_callback(
            "dragleave",
            self.dragleave.as_ref().unchecked_ref(),
        );
        let _ = self
            .element
            .remove_event_listener_with_callback("drop", self.drop.as_ref().unchecked_ref());
    }
}

pub fn use_uploader<'a>(cx: &'a ScopeState) -> &'a mut UseUploader {
    cx.use_hook(|| UseUploader::default())
}
