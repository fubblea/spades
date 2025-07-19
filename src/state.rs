use dioxus::logger::tracing;

#[derive(Debug, Clone, Default)]
pub struct Team {
    score: u32,
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

    pub fn get_team(&self, index: usize) -> Option<&Team> {
        self.teams.get(index)
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

    pub fn reset(&mut self) {
        self.round = 0;
        self.teams.clear();
        self.round_phase = RoundPhase::Setup;

        tracing::info!(
            "AppState reset: round {}, teams len {}",
            self.round,
            self.teams.len()
        );
    }
}
