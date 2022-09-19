use dioxus::{prelude::*, events::onclick};
use dioxus_free_icons::{Icon, icons::md_navigation_icons::*};

#[inline_props]
pub fn ArrawButton<'a>(cx: Scope, width: u32, height: u32, on_click: EventHandler<'a, bool>) -> Element {
    let action_enum = use_state(&cx, || false);
    let icon = if *action_enum.get() { 
        cx.render(rsx!{
            Icon {
                width: *width,
                height: *height,
                fill: "black",
                icon: MdArrowDropUp
            }
        }) 
    } else {
        cx.render(rsx!{
            Icon {
                width: *width,
                height: *height,
                fill: "black",
                icon: MdArrowDropDown
            }
        }) 
    };
    cx.render(rsx!{
        div {
            onclick: move |_| {
                action_enum.set(!action_enum);
                on_click.call(*action_enum.get());
            },
            icon
        }
    })
}