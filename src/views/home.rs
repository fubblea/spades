use crate::components::BigButton;
use crate::components::Incrementer;
use crate::components::Title;
use crate::Route;
use crate::APP_STATE;
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
                class: "flex-1 flex flex-col justify-center items-center p-8",
                div {
                    class: "p-8",
                    p {
                        class: "pb-4",
                        "Number of Teams:"
                    }
                    Incrementer {
                        value: APP_STATE.read().get_teams().len() as u32,
                        on_decrement: move |_| APP_STATE.write().remove_team(),
                        on_increment: move |_| APP_STATE.write().add_team(),
                    }
                }
                BigButton {
                    label: "Start Game".to_string(),
                    on_click: move |_| {
                        navigator.push(Route::Play {});
                    }
                }
            }
        }
    }
}
