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
}

impl AppState {
    pub fn new() -> Self {
        tracing::info!("AppState initialized");
        AppState {
            round: 1,
            round_phase: RoundPhase::Setup,
            teams: Vec::new(),
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

    pub fn reset(&mut self) {
        self.round = 1;
        self.teams.clear();
        self.round_phase = RoundPhase::Setup;

        tracing::info!(
            "AppState reset: round {}, teams len {}",
            self.round,
            self.teams.len()
        );
    }
}
