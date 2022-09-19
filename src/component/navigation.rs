use dioxus::{prelude::*, events::onmouseenter};

use crate::component::line_nav::LineButtonNav;
#[derive(Props)]
pub struct NavigationProps<'a>{
    pub normalStyle: &'a str,
    pub hoverStyle: &'a str,
    pub checkStyle: &'a str,
    pub defaultName: &'a str,
    pub space_padding: u8,
    pub direction: &'a str,
    pub childrens: Vec<&'a str>,
    pub on_navclick: EventHandler<'a, &'a str>,
}

pub fn Navigation<'a>(cx: Scope<'a, NavigationProps<'a>>) -> Element {
    let chilidren = cx.props.childrens.to_owned();
    let state = use_state(&cx, || cx.props.defaultName.to_string());
    cx.render(rsx!{
        div {
            style: "width:100%;height:100%;display:flex;flex-direction:{cx.props.direction}",
            chilidren.iter().enumerate().map( |(_, &value)|{
                rsx!(
                    LineButtonNav {
                        name: value,
                        hoverStyle: cx.props.hoverStyle,
                        normalStyle: cx.props.normalStyle,
                        checkStyle: cx.props.checkStyle,
                        margin: cx.props.space_padding,
                        isChecked: value.to_string() == *state.get(),
                        on_click: move |evt: &str| {
                            state.set(evt.to_string());
                            cx.props.on_navclick.call(evt);
                        },
                    }
                )
            })
        }
    })
}
