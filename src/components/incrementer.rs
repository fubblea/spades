use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct IncrementerProps {
    class: Option<String>,
    value: u32,
    on_increment: EventHandler<MouseEvent>,
    on_decrement: EventHandler<MouseEvent>,
}

#[component]
pub fn Incrementer(props: IncrementerProps) -> Element {
    const DEFAULT_CLASS: &str = "flex items-center space-x-4";
    let class = match props.class {
        Some(v) => format!("{} {}", v, DEFAULT_CLASS),
        None => DEFAULT_CLASS.to_string(),
    };

    rsx! {
        div {
            class: class,
            button {
                class: "bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded",
                onclick: props.on_decrement,
                "-",
            }
            span {
                class: "text-xl",
                "{props.value}"
            }
            button {
                class: "bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded",
                onclick: props.on_increment,
                "+",
            }
        }
    }
}
