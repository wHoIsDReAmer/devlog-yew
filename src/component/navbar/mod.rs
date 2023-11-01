use yew::prelude::*;
use yew_router::prelude::Link;

use crate::js_bind;
use crate::router::Route;

#[function_component(NavBar)]
pub fn navbar() -> Html {
    let css = include_bytes!("style.css");
    let css_string = String::from_utf8_lossy(css).to_string();

    js_bind::set_head_css(css_string.as_str());
    let key = use_state(|| js_bind::localstorage()
        .get("currentDisplay")
        .unwrap_or(Some("dark".into()))
        .unwrap_or("dark".into()));

    let key_1 = key.clone();
    use_effect(move || {
        let _ = js_bind::document().body()
            .expect("no body!")
            .set_attribute("data-theme", (*key_1).as_str());
    });

    let change_view = Callback::from(move |_: MouseEvent| {
        let key_2 = key.clone();

        if *key_2 == "" || *key_2 == "white" {
            key_2.set("dark".into());
            let _ = js_bind::localstorage().set("currentDisplay", "dark");
        } else {
            key_2.set("white".into());
            let _ = js_bind::localstorage().set("currentDisplay", "white");
        }
    });

    html! {
        <ul class={"navlist"}>
            <li> {"CWLog"} </li>
            <ul class={"flex-right"}>
                <p class="button"> <Link<Route> to={Route::Posts}> {"📒 Posts"} </Link<Route>>  </p>
                // <p class="button"> <Link<Route> to={Route::NotFound}> {"🔎 Tags"} </Link<Route>> </p>
                <p class="button"> <Link<Route> to={Route::Info}> {"📋 Info"} </Link<Route>> </p>
                <p class="button" onclick={change_view}> {"🌓"} </p>
            </ul>
        </ul>
    }
}