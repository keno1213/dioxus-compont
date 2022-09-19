use dioxus::prelude::*;

use crate::component::navigation::Navigation;

pub fn App(cx: Scope) -> Element {
    let state = use_state(&cx, ||String::from(""));
    let heardItems = vec! {
        "Item1", "Item2", "Item3"
    };
    // let onNav = move |name: &str| {
    //     match name {
    //         "Item1" => {},
    //         "Item2" => {},
    //         "Item3"  => {},
    //         _ => {},
    //     }
    // };
    let content = 
            match state.get().as_str() {
            "Item1" => cx.render(rsx!{
                div {
                    "Item1"
                }
            }),
            "Item2" => cx.render(rsx!{
                div {
                    "Item2"
                }
            }),
            "Item3"  => cx.render(rsx!{
                div {
                    "Item3"
                }
            }),
            _ => cx.render(rsx!{
                div {
                    "NONE"
                }
            }),
        };
    cx.render(rsx!{
        div {
            style: "width:100%;height:100%;background-color:#dcdcdc;",
            div {
                style: "width:100%;height:30px;background-color:white;",
                Navigation {
                    childrens: heardItems,
                    defaultName: "电视墙",
                    direction: "row",
                    hoverStyle: "#0062fe",
                    normalStyle: "black",
                    checkStyle: "#0062fe",
                    space_padding: 20,
                    on_navclick: move |name: &str| state.set(name.to_string()),
                }
            }
            content
        }
    })
}