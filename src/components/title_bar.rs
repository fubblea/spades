use dioxus::prelude::*;

use crate::{components::Title, Route, APP_STATE};

#[derive(Props, Clone, PartialEq)]
pub struct TitleBarProps {
    class: Option<String>,
    title: String,
}

#[component]
pub fn TitleBar(props: TitleBarProps) -> Element {
    const DEFAULT_CLASS: &str = "w-full flex flex-row items-center";
    let class = match props.class {
        Some(v) => format!("{} {}", v, DEFAULT_CLASS),
        None => DEFAULT_CLASS.to_string(),
    };

    let navigator = use_navigator();

    rsx! {
        div {
            id: "top_bar",
            class: class,
            // Left spacer
            div {
                class: "flex-1",
            }
            // Centered title
            div {
                class: "flex-1 flex justify-center",
                Title {
                    text: props.title,
                }
            }
            // Quit button
            div {
                class: "flex-1 flex justify-end",
                div {
                    id: "quit_button",
                    class: "pr-3",
                    button {
                        class: "bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-6 rounded text-s",
                        onclick: move |_| {
                            navigator.push(Route::Home {});
                            APP_STATE.write().reset();
                        },
                        "Quit"
                    }
                }
            }
        }
    }
}
