use dioxus::{prelude::*, events::{onmouseleave, onmouseenter, onclick}};
///by keno1213
#[derive(Props)]
pub struct LineButtonProps<'a> {
    pub normalStyle: &'a str,
    pub hoverStyle: &'a str,
    pub checkStyle: &'a str,
    pub isChecked: bool,
    pub margin: u8,
    pub name: &'a str,
    pub on_click: EventHandler<'a, &'a str>,
}
pub fn LineButtonNav<'a>(cx: Scope<'a, LineButtonProps<'a>>) -> Element {
    let is_hovered = use_state(&cx, || false);
    let is_clicked = cx.props.isChecked;
    let hover_style = if *is_hovered.get() {
        format!("color:{};",cx.props.hoverStyle)
    } else {
        String::default()
    };
    let clicked_style = if is_clicked {
        format!("border-bottom:2px solid {};color:{};",cx.props.checkStyle, cx.props.checkStyle)
    } else {
        format!("color:{};",cx.props.normalStyle)
    };
    cx.render(rsx!{
        label {
            style: "width:100%;height:100%;text-align:center;margin:0px {cx.props.margin};{clicked_style};{hover_style}",
            onmouseover: move |_| {is_hovered.set(!is_hovered);},
            onmouseout: move |_| {is_hovered.set(!is_hovered);},
            onclick: move |_| {cx.props.on_click.call(cx.props.name);},
            "{cx.props.name}"
        }
    })
}