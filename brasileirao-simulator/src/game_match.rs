use std::fs;

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
        mut internacional_first_match_stats: [u32; 3],
    ) -> (Vec<Team>, [u32; 3]) {
        let mut rng = rand::thread_rng();
        let rng_one: f32 = rng.gen_range(0.0..1.0);
        let rng_two: f32 = rng.gen_range(0.0..1.0);

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
            if first_team.name == "Internacional" && first_team.games == 12 {
                match (first_win_rate >= rng_one, second_win_rate >= rng_two) {
                    (true, false) => {
                        internacional_first_match_stats[0] += 1;
                        (first_team.win_points(), second_team.lose())
                    }
                    (false, true) => {
                        internacional_first_match_stats[2] += 1;
                        (first_team.lose(), second_team.win_points())
                    }
                    _ => {
                        internacional_first_match_stats[1] += 1;
                        (first_team.tie_points(), second_team.tie_points())
                    }
                }
            } else if second_team.name == "Internacional" && second_team.games == 12 {
                match (first_win_rate >= rng_one, second_win_rate >= rng_two) {
                    (true, false) => {
                        internacional_first_match_stats[2] += 1;
                        (first_team.win_points(), second_team.lose())
                    }
                    (false, true) => {
                        internacional_first_match_stats[0] += 1;
                        (first_team.lose(), second_team.win_points())
                    }
                    _ => {
                        internacional_first_match_stats[1] += 1;
                        (first_team.tie_points(), second_team.tie_points())
                    }
                }
            } else {
                match (first_win_rate >= rng_one, second_win_rate >= rng_two) {
                    (true, false) => (first_team.win_points(), second_team.lose()),
                    (false, true) => (first_team.lose(), second_team.win_points()),
                    _ => (first_team.tie_points(), second_team.tie_points()),
                }
            };

        teams_vec[first_team_index] = first_team;
        teams_vec[second_team_index] = second_team;
        (teams_vec, internacional_first_match_stats)
    }
}

pub fn initialize_match_vec() -> Vec<Match> {
    let mut match_vec: Vec<Match> = Vec::with_capacity(380);
    let content = fs::read_to_string("../jogos13-06.txt").expect("Deve existir esse arquivo.");
    for part in content.lines() {
        let current_match = Match::new(part);
        match_vec.push(current_match);
    }
    match_vec
}
