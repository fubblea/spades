use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BigButtonProps {
    class: Option<String>,
    label: String,
    on_click: EventHandler<MouseEvent>,
}

#[component]
pub fn BigButton(props: BigButtonProps) -> Element {
    const DEFAULT_CLASS: &str =
        "bg-blue-500 hover:bg-blue-700 text-white font-bold py-4 px-8 rounded text-xl";
    let class = match props.class {
        Some(v) => format!("{} {}", v, DEFAULT_CLASS),
        None => DEFAULT_CLASS.to_string(),
    };
    rsx! {
        button {
            class: class,
            onclick: props.on_click,
            "{props.label}"
        }
    }
}
