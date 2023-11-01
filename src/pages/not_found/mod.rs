use yew::prelude::*;
use crate::js_bind;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    js_bind::set_title("Devlog | Not found");

    html! {
        <div class={"container"}>
            <div class={"inner"}>
                <h3> {"존재하지 않는 페이지인데요.."} </h3>
            </div>
        </div>
    }
}