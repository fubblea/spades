use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct TypingProps {
    class: Option<String>,
    text: String,
}

#[component]
pub fn Title(props: TypingProps) -> Element {
    const DEFAULT_CLASS: &str = "flex justify-center items-center";
    let class = match props.class {
        Some(v) => format!("{} {}", v, DEFAULT_CLASS),
        None => DEFAULT_CLASS.to_string(),
    };

    rsx! {
        title {
            class: "{class}",
            "{props.text}"
        }
    }
}
