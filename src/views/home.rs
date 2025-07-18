use crate::components::BigButton;
use crate::components::Title;
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
                Title {
                    text: "Spades Scorer".to_string(),
                },
                p {
                    class: "text-center",
                    "A simple app to keep track of your Spades game scores."
                }
            }
            div {
                class: "flex-1 flex justify-center items-center p-8",
                BigButton {
                    label: "New Game".to_string(),
                    on_click: move |_| {
                        navigator.push(Route::Setup {});
                    }
                }
            }
        }
    }
}
