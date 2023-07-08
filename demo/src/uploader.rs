use dioxus::core::ScopeState;

pub struct UseUploader;

impl UseUploader {
    fn new() -> Self {
        UseUploader
    }
}

impl Drop for UseUploader {
    fn drop(&mut self) {
        // TODO
    }
}

pub fn use_uploader(cx: &ScopeState) -> &UseUploader {
    cx.use_hook(|| UseUploader::new())
}
