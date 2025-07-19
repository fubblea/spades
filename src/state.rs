use dioxus::logger::tracing;

#[derive(PartialEq, Debug, Clone, Default)]
pub struct Team {
    pub contract: u32,
    pub score: u32,
    pub rounds_won: u32,
    pub round_hands: u32,
    pub delta_leader: i32,
}

#[derive(Debug, Clone, Default)]
pub enum RoundPhase {
    #[default]
    Setup,
    Play,
}

#[derive(Debug, Default)]
pub struct AppState {
    round: u32,
    round_phase: RoundPhase,
    teams: Vec<Team>,
    leader_score: u32,
}

impl AppState {
    pub fn new() -> Self {
        tracing::info!("AppState initialized");
        AppState {
            round: 1,
            round_phase: RoundPhase::Setup,
            teams: Vec::new(),
            leader_score: 0,
        }
    }

    pub fn get_teams(&self) -> &Vec<Team> {
        &self.teams
    }

    pub fn add_team(&mut self) {
        self.teams.push(Team::default());
        tracing::info!("Added team, new teams len: {}", self.teams.len());
    }

    pub fn remove_team(&mut self) {
        tracing::info!("Removed team, new teams len: {}", self.teams.len());
        self.teams.pop();
    }

    pub fn get_round(&self) -> u32 {
        self.round
    }

    pub fn next_round(&mut self) {
        self.update_round_scores();
        self.update_delta_leader();

        self.round += 1;
        self.round_phase = RoundPhase::Setup;
        tracing::info!("Next round: {}", self.round);
    }

    pub fn get_round_phase(&self) -> &RoundPhase {
        &self.round_phase
    }

    pub fn start_round(&mut self) {
        self.round_phase = RoundPhase::Play;
        tracing::info!("Round started: {}", self.round);
    }

    pub fn update_team_contract(&mut self, idx: usize, contract: u32) {
        if let Some(team) = self.teams.get_mut(idx) {
            team.contract = contract;
            tracing::info!("Updated team {} contract to {}", idx + 1, contract);
        } else {
            tracing::warn!(
                "Attempted to update contract for non-existent team {}",
                idx + 1
            );
        }
    }

    pub fn team_won_hand(&mut self, idx: usize) {
        if let Some(team) = self.teams.get_mut(idx) {
            team.round_hands += 1;
            tracing::info!(
                "Noted team {} won hand, won hands now: {}",
                idx + 1,
                team.round_hands
            );
        } else {
            tracing::warn!(
                "Attempted to update won hands for non-existent team {}",
                idx + 1
            );
        }
    }

    fn update_round_scores(&mut self) {
        self.leader_score = 0;
        let mut round_winner: Option<usize> = None;

        self.teams.iter_mut().enumerate().for_each(|(i, team)| {
            if team.round_hands >= team.contract {
                team.score += 10 * team.contract + (team.round_hands - team.contract);
            }

            if team.score > self.leader_score {
                self.leader_score = team.score;
                round_winner = Some(i);
            }

            team.round_hands = 0; // Reset hands won for the next round
        });

        match round_winner {
            Some(winner_idx) => {
                self.teams[winner_idx].rounds_won += 1;
                tracing::info!(
                    "Team {} won the round with score {}",
                    winner_idx + 1,
                    self.teams[winner_idx].score
                );
            }
            None => tracing::warn!("No team won the round"),
        }
    }

    fn update_delta_leader(&mut self) {
        self.teams.iter_mut().for_each(|team| {
            team.delta_leader = team.score as i32 - self.leader_score as i32;
            tracing::info!(
                "Team score: {}, Delta Leader: {}",
                team.score,
                team.delta_leader
            );
        });
    }

    pub fn reset(&mut self) {
        self.round = 1;
        self.teams.clear();
        self.round_phase = RoundPhase::Setup;
        self.leader_score = 0;

        tracing::info!(
            "AppState reset: round {}, teams len {}",
            self.round,
            self.teams.len()
        );
    }
}
