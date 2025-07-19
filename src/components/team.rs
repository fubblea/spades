use dioxus::prelude::*;

use crate::{
    components::{BigButton, Incrementer},
    state::{RoundPhase, Team},
    APP_STATE,
};

#[derive(Props, Clone, PartialEq)]
pub struct TeamProps {
    class: Option<String>,
    idx: usize,
}

#[component]
pub fn TeamComponent(props: TeamProps) -> Element {
    const DEFAULT_CLASS: &str =
        "flex flex-row p-4 border-4 rounded shadow-md items-center justify-center";
    let class = match props.class {
        Some(v) => format!("{} {}", v, DEFAULT_CLASS),
        None => DEFAULT_CLASS.to_string(),
    };

    let update_team = move |_| {
        match APP_STATE.read().get_teams().get(props.idx) {
            Some(team) => team.clone(),
            None => Team::default(), // Fallback to a default team if not found
        }
    };
    let mut team = use_signal(|| update_team(props.idx.clone()));

    use_effect(move || {
        *team.write() = update_team(props.idx);
    });

    rsx! {
        div {
            class: class,
            div {
                class : "flex flex-col items-center",
                p {
                    class: "pd-2",
                    "Contract:"
                }
                {
                    match APP_STATE.read().get_round_phase() {
                        RoundPhase::Setup => rsx! {
                            div {
                                class: "p-4",
                                Incrementer{
                                    value: team.read().contract,
                                    on_decrement: move |_| {
                                        if team.read().contract > 0 {
                                            APP_STATE.write().update_team_contract(props.idx, team.read().contract - 1);
                                        }
                                        *team.write() = update_team(props.idx);
                                    },
                                    on_increment: move |_| {
                                        APP_STATE.write().update_team_contract(props.idx, team.read().contract + 1);
                                        *team.write() = update_team(props.idx);
                                    },
                                }
                            }
                        },
                        RoundPhase::Play => rsx! {
                            span {
                                class: "text-xl py-2",
                                "{team.read().contract}"
                            }
                        },
                    }
                }
                div {
                    class: "p-4",
                    {
                        match APP_STATE.read().get_round_phase() {
                            RoundPhase::Setup => rsx! {
                            },
                            RoundPhase::Play => rsx! {
                                BigButton{
                                   label: "Won Hand",
                                   on_click: move |_| {
                                       APP_STATE.write().team_won_hand(props.idx);
                                       *team.write() = update_team(props.idx);
                                    }
                                }
                            },
                        }
                    }
                }
            }
            div {
                class: "flex flex-col border",
                div {
                    class: "p-2",
                    p {
                        class: "pd-2",
                        "Score: {team.read().score}"
                    }
                    p {
                        class: "pd-2",
                        "Hands Won This Round: {team.read().round_hands}"
                    }
                    p {
                        class: "pd-2",
                        "Rounds Won: {team.read().rounds_won}"
                    }
                    p {
                        class: "pd-2",
                        "Delta Leader: {team.read().delta_leader}"
                    }
                }
            }
        }
    }
}
