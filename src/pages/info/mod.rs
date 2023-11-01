use yew::prelude::*;
use crate::js_bind;

#[function_component(Info)]
pub fn info() -> Html {
    js_bind::set_title("Devlog | Info");

    html! {
        <div class={"container"}>
            <div class={"inner"}>
                <h3> {"Info 페이지요"} </h3>
            </div>
        </div>
    }
}