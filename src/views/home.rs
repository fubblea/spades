use crate::components::BigButton;
use crate::Route;
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    let navigator = use_navigator();
    rsx! {
        div {
            class: "flex flex-col h-[80vh]",
            div {
                title {
                    class: "flex justify-center items-center",
                    "Spades"
                }
                p {
                    class: "text-center text-lg",
                    "A simple spades game built with Dioxus and Rust. Select a mode to start playing or scoring a game."
                }
            }

            div {
                class: "flex flex-1 flex-row items-center justify-center gap-4",
                BigButton {
                    label: "Play Game".to_string(),
                    on_click: move |_| {
                        navigator.push(Route::PlayGameSetup {});
                    }
                }
                BigButton {
                    label: "Score Game".to_string(),
                    on_click: move |_| {
                        navigator.push(Route::ScoreGameSetup {});
                    }
                }
            }
        }
    }
}
