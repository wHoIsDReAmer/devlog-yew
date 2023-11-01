mod router;
mod pages;
mod js_bind;
mod component;

use yew::prelude::*;

fn main() {
    yew::Renderer::<router::RouterComponent>::new().render();
}