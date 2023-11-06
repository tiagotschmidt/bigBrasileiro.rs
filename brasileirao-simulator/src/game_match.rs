use std::{
    fmt::Display,
    fs::{self},
};

use crate::team::Team;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Match {
    pub first_team: String,
    pub second_team: String,
    date: String,
}

impl Display for Match {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{};{};{}", self.date, self.first_team, self.second_team)
    }
}

impl Match {
    pub fn new(match_string: &str) -> Self {
        let subparts = match_string.split(';').collect::<Vec<_>>();
        let date = subparts[0];
        let first_team = subparts[1];
        let second_team = subparts[2];

        Match {
            first_team: first_team.to_string(),
            second_team: second_team.to_string(),
            date: date.to_string(),
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
        first_match_index: u32,
        mut teams_vec: Vec<Team>,
        mut internacional_first_match_stats: [u32; 3],
    ) -> (Vec<Team>, [u32; 3]) {
        let mut rng = rand::thread_rng();

        let (first_team_index, second_team_index) =
            Match::find_teams(self.first_team, self.second_team, teams_vec.clone());

        let first_team = teams_vec[first_team_index].clone();
        let second_team = teams_vec[second_team_index].clone();

        let first_win_rate = if first_team.win_rate == 0.0 {
            rng.gen_range(0.0..0.3)
        } else {
            first_team.win_rate
        };

        let second_win_rate = if second_team.win_rate == 0.0 {
            rng.gen_range(0.0..0.3)
        } else {
            second_team.win_rate
        };

        let (first_team, second_team) = loop {
            let rng_one: f32 = rng.gen_range(0.0..1.0);
            let rng_two: f32 = rng.gen_range(0.0..1.0);

            if first_team.name == "Internacional" && first_team.games == first_match_index {
                match (first_win_rate >= rng_one, second_win_rate >= rng_two) {
                    (true, false) => {
                        internacional_first_match_stats[0] += 1;
                        break (first_team.win_points(), second_team.lose());
                    }
                    (false, true) => {
                        internacional_first_match_stats[2] += 1;
                        break (first_team.lose(), second_team.win_points());
                    }
                    (false, false) => {
                        internacional_first_match_stats[1] += 1;
                        break (first_team.tie_points(), second_team.tie_points());
                    }
                    (true, true) => {}
                }
            } else if second_team.name == "Internacional" && second_team.games == first_match_index
            {
                match (first_win_rate >= rng_one, second_win_rate >= rng_two) {
                    (true, false) => {
                        internacional_first_match_stats[2] += 1;
                        break (first_team.win_points(), second_team.lose());
                    }
                    (false, true) => {
                        internacional_first_match_stats[0] += 1;
                        break (first_team.lose(), second_team.win_points());
                    }
                    (false, false) => {
                        internacional_first_match_stats[1] += 1;
                        break (first_team.tie_points(), second_team.tie_points());
                    }
                    (true, true) => {}
                }
            } else {
                match (first_win_rate >= rng_one, second_win_rate >= rng_two) {
                    (true, false) => break (first_team.win_points(), second_team.lose()),
                    (false, true) => break (first_team.lose(), second_team.win_points()),
                    (false, false) => break (first_team.tie_points(), second_team.tie_points()),
                    (true, true) => {}
                }
            }
        };

        teams_vec[first_team_index] = first_team;
        teams_vec[second_team_index] = second_team;
        (teams_vec, internacional_first_match_stats)
    }
}

pub fn initialize_match_vec() -> Vec<Match> {
    let mut match_vec: Vec<Match> = Vec::with_capacity(380);
    let content = fs::read_to_string("../jogos06-11.txt").expect("Deve existir esse arquivo.");
    for part in content.lines() {
        let current_match = Match::new(part);
        match_vec.push(current_match);
    }
    match_vec
}
