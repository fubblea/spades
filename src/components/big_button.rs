use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BigButtonProps {
    label: String,
    on_click: EventHandler<MouseEvent>,
}

#[component]
pub fn BigButton(props: BigButtonProps) -> Element {
    rsx! {
        button {
            class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-4 px-8 rounded text-xl",
            onclick: props.on_click,
            "{props.label}"
        }
    }
}
