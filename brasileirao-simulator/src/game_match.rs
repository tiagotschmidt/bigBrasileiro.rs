use crate::team::Team;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Match {
    pub first_team: String,
    pub second_team: String,
}

impl Match {
    pub fn new(match_string: &str) -> Self {
        let subparts = match_string.split(' ').collect::<Vec<_>>();
        let first_team = subparts[2];
        let second_team = subparts[4];

        Match {
            first_team: first_team.to_string(),
            second_team: second_team.to_string(),
        }
    }

    pub fn find_teams(
        first_name: String,
        second_name: String,
        teams_vec: Vec<Team>,
    ) -> (usize, usize) {
        let first = teams_vec
            .iter()
            .enumerate()
            .find(|(_index, item)| item.name == first_name)
            .map(|(index, _item)| index)
            .unwrap();
        let second = teams_vec
            .iter()
            .enumerate()
            .find(|(_index, item)| item.name == second_name)
            .map(|(index, _item)| index)
            .unwrap();

        (first, second)
    }

    pub fn simulate_points_game(
        self,
        mut teams_vec: Vec<Team>,
        mut match_history: Vec<MatchRegistry>,
    ) -> (Vec<Team>, Vec<MatchRegistry>) {
        let mut rng = rand::thread_rng();
        let rng_one: f32 = rng.gen_range(0.0..1.0);
        let rng_two: f32 = rng.gen_range(0.0..1.0);
        let mut match_status = MatchStatus::Undefined;

        let (first_team_index, second_team_index) =
            Match::find_teams(self.first_team, self.second_team, teams_vec.clone());

        let first_team = teams_vec[first_team_index].clone();
        let second_team = teams_vec[second_team_index].clone();

        let first_win_rate = if first_team.win_rate == 0.0 {
            rng.gen_range(0.0..0.5)
        } else {
            first_team.win_rate
        };

        let second_win_rate = if second_team.win_rate < 0.1 {
            rng.gen_range(0.0..0.5)
        } else {
            second_team.win_rate
        };

        let (first_team, second_team) =
            match (first_win_rate >= rng_one, second_win_rate >= rng_two) {
                (true, false) => {
                    match_status = MatchStatus::FirstWon;
                    (first_team.win_points(), second_team.lose())
                }
                (false, true) => {
                    match_status = MatchStatus::SecondWon;
                    (first_team.lose(), second_team.win_points())
                }
                _ => {
                    match_status = MatchStatus::Tie;
                    (first_team.tie_points(), second_team.tie_points())
                }
            };

        if first_team.name == "Internacional" || second_team.name == "Internacional" {
            match_history.push(MatchRegistry::new(
                first_team.name.clone(),
                second_team.name.clone(),
                match_status,
            ));
        };

        teams_vec[first_team_index] = first_team;
        teams_vec[second_team_index] = second_team;

        (teams_vec, match_history)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SequenceCounter {
    match_history: Vec<MatchRegistry>,
    repeated: u32,
}

impl SequenceCounter {
    pub fn new(match_history: Vec<MatchRegistry>) -> Self {
        SequenceCounter {
            match_history,
            repeated: 0,
        }
    }

    pub fn raise_counter(&mut self) {
        self.repeated += 1;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchRegistry {
    first_team: String,
    second_team: String,
    match_status: MatchStatus,
}

impl MatchRegistry {
    pub fn new(first_team: String, second_team: String, match_status: MatchStatus) -> Self {
        MatchRegistry {
            first_team,
            second_team,
            match_status,
        }
    }
}

impl Default for MatchRegistry {
    fn default() -> Self {
        MatchRegistry {
            first_team: "".to_string(),
            second_team: "".to_string(),
            match_status: MatchStatus::Undefined,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MatchStatus {
    FirstWon,
    SecondWon,
    Tie,
    Undefined,
}
