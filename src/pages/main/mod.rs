use yew::prelude::*;
use crate::js_bind::set_title;

#[function_component(Main)]
pub fn main_page() -> Html {
    html! {
        <h1>{"main"}</h1>
    }
}