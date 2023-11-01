use yew::{function_component, html, Html};
use yew_router::prelude::*;

use crate::component::navbar::NavBar;
use crate::pages::{main::Main,
                   posts::Posts,
                   info::Info,
                   login::Login,
                   not_found::NotFound};
use crate::js_bind;
use crate::js_bind::{document, localstorage};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Index,
    #[at("/posts")]
    Posts,
    #[at("/info")]
    Info,
    #[at("/login")]
    Login,
    #[at("/*")]
    NotFound
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Index => html! { <Main/> },
        Route::Posts => html! { <Posts/> },
        Route::Info => html! { <Info/> },
        Route::Login => html! { <Login/> },
        Route::NotFound => html! { <NotFound/> }
    }
}

#[function_component(RouterComponent)]
pub fn main_router() -> Html {
    let css_byte = include_bytes!("global.css");
    let css = String::from_utf8_lossy(css_byte).to_string();

    let theme = localstorage().get("data-theme").unwrap_or(Some("".into()));

    let _ = document().body()
        .expect("no body!")
        .set_attribute("data-theme", theme.unwrap_or("dark".into()).as_str());

    js_bind::set_head_css(css.as_str());
    html! {
        <BrowserRouter>
            <NavBar/>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}