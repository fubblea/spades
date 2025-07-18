use dioxus::logger::tracing;

#[derive(Debug, Clone, Default)]
pub struct Team {
    score: u32,
}

#[derive(Debug, Default)]
pub struct AppState {
    teams: Vec<Team>,
}

impl AppState {
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
}
