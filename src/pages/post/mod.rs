use yew::prelude::*;
use web_sys::window;

#[derive(PartialEq, Properties, Clone)]
pub struct PostProps {
    pub id: String,
}

#[function_component(Post)]
pub fn post(props: &PostProps) -> Html {
    // Convert markdown to HTML
    let markdown_html = markdown::to_html("# Hello world\n\n## HIHI\n|Foo|bar|\n|---|---|\n|some|test|");

    let id_clone = props.id.clone();
    use_effect(move || {
        let window = window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        if let Some(element) = document.get_element_by_id(&id_clone) {
            element.set_inner_html(&markdown_html);
        }
    });

    html! {
        <div class={"container"}>
            <div id={props.id.clone()} class={"inner"}>
            </div>
        </div>
    }
}
