use dioxus::prelude::*;

use crate::components::Title;

#[component]
pub fn Play() -> Element {
    rsx! {
        Title {
            text: "Play".to_string(),
        }
    }
}
