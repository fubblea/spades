use dioxus::{logger::tracing, prelude::*};

use crate::{
    components::{TeamComponent, TitleBar},
    state::RoundPhase,
    APP_STATE,
};

#[component]
pub fn Play() -> Element {
    const NUM_COLS: usize = 3;

    let update_num_teams = || APP_STATE.read().get_teams().len();
    let num_teams = use_signal(|| update_num_teams());

    let num_rows = use_signal(|| (*num_teams.read() as f32 / NUM_COLS as f32).ceil() as usize);
    tracing::info!("Num rows: {}", num_rows);

    let update_title = || format!("Round: {}", APP_STATE.read().get_round());
    let mut title = use_signal(|| update_title());

    let update_subtitle = || match APP_STATE.read().get_round_phase() {
        RoundPhase::Setup => "Setup (Set contracts and click 'Start Round' to begin)",
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
            class: "flex flex-col items-center justify-center p-4 w-full",
            div {
                class: format!(
                    "w-full grid grid-rows-{} grid-cols-{} gap-4 items-center justify-center",
                    NUM_COLS, *num_rows.read() // TODO: Fix this
                )
                ,
                {APP_STATE.read().get_teams().iter().enumerate().map(|(i, _)| rsx! {
                    div {
                        id: format!("team_{}", i + 1),
                        class: "flex flex-col p-4",
                        h1 { "Team: {i+1}" }
                        TeamComponent{idx: i}
                    }
                })}
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
