use std::collections::HashSet;
use std::fmt::Debug;
use std::time;
use yew::prelude::*;
use crate::js_bind::{set_head_css, set_title};
use wasm_bindgen::JsCast;

#[derive(Properties, PartialEq, Clone)]
struct PostProp {
    image_link: String,
    title: String,
    desc: String,
    date: String,
    tags: Vec<String>
}

#[function_component(PostItem)]
fn post_item(props: &PostProp) -> Html {
    let props_clone = props.clone();

    html! {
        <div class={"post"}>
            <img src={props_clone.image_link} alt={"desc"} class={"post-image"}/>
            <p class={"title"}> {props_clone.title} </p>
            <p class={"desc"}> {props_clone.desc} </p>
            <p class={"date"}> {props_clone.date} </p>
            <ul class={"tags"}>
                {
                    for props_clone.tags.iter().map(|v| html! {
                        <li> {v} </li>
                    })
                }
            </ul>
        </div>
    }
}

#[function_component(Posts)]
pub fn post() -> Html {
    set_title("Devlog | Posts");

    let css_bytes = include_bytes!("style.css");
    let css = String::from_utf8_lossy(css_bytes).to_string();
    set_head_css(css.as_str());

    let selected_tag = use_state(|| String::from(""));
    let selected_tag2 = selected_tag.clone();

    let mock_data = vec! {
        PostProp {
            title: "Hello".into(),
            desc: "WTT".into(),
            date: "2022-08-13".into(),
            image_link: "https://images.unsplash.com/photo-1608848461950-0fe51dfc41cb?auto=format&fit=crop&q=80&w=1000&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxleHBsb3JlLWZlZWR8Mnx8fGVufDB8fHx8fA%3D%3D".into(),
            tags: vec!["Yew".into()]
        },
        PostProp {
            title: "Rust is Awesome".into(),
            desc: "Discover the power of Rust BlahBlahBlahBlahBlahBlahBlahBlahBlah".into(),
            date: "2022-08-14".into(),
            image_link: "https://images.unsplash.com/photo-1608848461950-0fe51dfc41cb?auto=format&fit=crop&q=80&w=1000&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxleHBsb3JlLWZlZWR8Mnx8fGVufDB8fHx8fA%3D%3D".into(),
            tags: vec!["Rust".into(), "Programming".into()]
        },
        PostProp {
            title: "Getting Started with Yew".into(),
            desc: "A beginner's guide to Yew BlahBlahBlahBlahBlahBlah".into(),
            date: "2022-08-15".into(),
            image_link: "https://images.unsplash.com/photo-1608848461950-0fe51dfc41cb?auto=format&fit=crop&q=80&w=1000&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxleHBsb3JlLWZlZWR8Mnx8fGVufDB8fHx8fA%3D%3D".into(),
            tags: vec!["Yew".into(), "WebAssembly".into()]
        },
        PostProp {
            title: "WebAssembly Magic".into(),
            desc: "Dive into the world of WebAssembly BlahBlahBlahBlahBlahBlah".into(),
            date: "2022-08-16".into(),
            image_link: "https://images.unsplash.com/photo-1608848461950-0fe51dfc41cb?auto=format&fit=crop&q=80&w=1000&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxleHBsb3JlLWZlZWR8Mnx8fGVufDB8fHx8fA%3D%3D".into(),
            tags: vec!["WebAssembly".into(), "Performance".into()]
        },
    };

    let mut distinct: HashSet<String> = HashSet::new();

    html! {
        <div class={"container"}>
            <div class={"inner"}>
                <p>{"태그"}</p>
                <ul class={"tags"}>
                    {
                        for mock_data.iter().map(move |m| {
                            html! {
                                {
                                    for m.tags.iter().map(|p| {
                                        if distinct.contains(p) {
                                            return html! {}
                                        }

                                        distinct.insert(p.clone());

                                        let selected_tag_clone = selected_tag.clone();
                                        let tag_clicked = Callback::from(move |e: MouseEvent| {
                                            let target = e.target().unwrap();
                                            let target_text = target.dyn_into::<web_sys::HtmlElement>().expect("Cannot casting to HtmlElement").inner_text();

                                            if selected_tag_clone.to_string() == target_text {
                                                selected_tag_clone.set("".into());
                                            } else {
                                                selected_tag_clone.set(target_text);
                                            }
                                        });

                                        let selected_tag_clone = selected_tag.clone();
                                        html! {
                                            <li class={
                                                if selected_tag_clone.to_string().eq(p) {
                                                    "tag-enabled"
                                                } else {
                                                    ""
                                                }
                                            } onclick={tag_clicked}>{p}</li>
                                        }
                                    })
                                }
                            }
                        })
                    }
                 </ul>

                <div class={"posts"}>
                    {
                        for mock_data.iter().map(move |p| {
                            let selected_tag_clone = selected_tag2.clone();

                            let p_clone = p.clone();
                            if selected_tag_clone.to_string() != "" && !p.tags.contains(&selected_tag_clone.to_string()) {
                                return html! {}
                            }

                            html! {
                                <PostItem
                                    title={p_clone.title}
                                    desc={p_clone.desc}
                                    date={p_clone.date}
                                    image_link={p_clone.image_link}
                                    tags={p_clone.tags}/>
                            }
                        })
                    }
                </div>
            </div>
        </div>
    }
}