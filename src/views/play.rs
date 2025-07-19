use dioxus::prelude::*;

use crate::{components::TitleBar, state::RoundPhase, APP_STATE};

#[component]
pub fn Play() -> Element {
    let update_title = || format!("Round: {}", APP_STATE.read().get_round());
    let mut title = use_signal(|| update_title());

    let update_subtitle = || match APP_STATE.read().get_round_phase() {
        RoundPhase::Setup => "Setup",
        RoundPhase::Play => "Playing",
    };
    let mut subtitle = use_signal(|| update_subtitle());

    let update_button_text = || match APP_STATE.read().get_round_phase() {
        RoundPhase::Setup => "Start Round",
        RoundPhase::Play => "Next Round",
    };
    let mut button_text = use_signal(|| update_button_text());

    let update_round_phase = || APP_STATE.read().get_round_phase().clone();
    let mut round_phase = use_signal(|| update_round_phase());

    rsx! {
        TitleBar {
            title: title.read(),
        }
        h2 {
            class: "text-center text-xl",
            "{subtitle.read()}"
        }

        div {
            id: "play_area",
            class: "flex flex-col items-center justify-center p-4",
            div {
                class: "grid grid-cols-2 grid-rows-2 gap-4 w-full max-w-md mb-8 h-[65vh]",
                h1 {"Hi"}
                h1 {"Hi"}
                h1 {"Hi"}
                h1 {"Hi"}
            }
            div {
                class: "w-full flex justify-center mt-4",
                button {
                    class: "px-4 py-2 bg-blue-500 text-white rounded bg-blue-500 hover:bg-blue-700",
                    onclick: move |_| {
                        match *round_phase.read() {
                            RoundPhase::Setup => {
                                APP_STATE.write().start_round();
                            },
                            RoundPhase::Play => {
                                APP_STATE.write().next_round();
                            },
                        }
                        *title.write() = update_title();
                        *subtitle.write() = update_subtitle();
                        *button_text.write() = update_button_text();
                        *round_phase.write() = update_round_phase();
                    },
                    "{button_text.read()}"
                }
            }
        }
    }
}
