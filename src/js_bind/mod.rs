use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlElement, Storage, window};

pub fn set_title(title: &str) {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    document.set_title(title);
}

pub fn set_head_css(sty: &str) {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let head = document.head().expect("document should have a head");

    let style = document.create_element("style").expect("Failed to create style element");
    style.set_inner_html(sty);

    let style = style.dyn_into::<HtmlElement>().expect("Failed to cast element");
    head.append_child(&style).expect("Failed to append style to head");
}

pub fn localstorage() -> Storage {
    window().expect("no global `window` exists")
        .local_storage()
        .expect("no global `localStorage` exists")
        .expect("cannot get localStorage")
}

pub fn document() -> Document {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    document
}