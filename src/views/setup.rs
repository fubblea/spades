use crate::components::Title;
use dioxus::prelude::*;

#[component]
pub fn Setup() -> Element {
    rsx! {
        div {
            class: "flex flex-col h-[80vh]",
            div {
                Title {
                    text: "Score Game Setup".to_string(),
                }
            }
        }
    }
}
