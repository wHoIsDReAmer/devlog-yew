use yew::prelude::*;
use crate::js_bind::set_head_css;

#[function_component(Login)]
pub fn login() -> Html {
    let css = include_bytes!("style.css");
    let css_string = String::from_utf8_lossy(css).to_string();

    set_head_css(css_string.as_str());

    html! {
        <div class={"container"}>
            <div class={"inner"}>
                <div class={"login-form"}>
                    <input/>
                    <input/>
                    <button>{"로그인"}</button>
                </div>
            </div>
        </div>
    }
}