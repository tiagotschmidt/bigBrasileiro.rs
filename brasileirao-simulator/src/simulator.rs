use std::{collections::HashMap, fs, sync::Arc, thread, vec};

use crate::{game_match::Match, team::Team, MAX_SIM, MAX_THREADS, NUMBER_OF_TEAMS};

pub struct Simulator {
    pub team_vec: Vec<Team>,
    pub match_vec: Vec<Match>,
    pub observed_teams: Vec<String>,
}

impl Simulator {
    pub fn new(observed_teams: Vec<String>) -> Self {
        let team_vec: Vec<Team> = Vec::with_capacity(20);
        let match_vec: Vec<Match> = Vec::with_capacity(380);

        Simulator {
            team_vec,
            match_vec,
            observed_teams,
        }
    }

    fn assert_points(team: &Team) -> bool {
        let file_input_points = team.points;

        let current_ties_and_loses = team.games - team.wins;

        let sum_of_all_possible_points = team.wins * 3 + current_ties_and_loses;

        file_input_points <= sum_of_all_possible_points //This happens because our model does not
                                                        //differentiate lose and tie. Therefore, our
                                                        //right hand estimative is always the input
                                                        //amount or more (considering all not wind ==
                                                        //ties).
    }

    fn valide_team_vec(self) -> bool {
        self.team_vec.iter().all(Self::assert_points)
    }

    fn read_team_from_line(part: &str, i: usize) -> Team {
        let subparts = part.split(';').collect::<Vec<_>>();

        let name = subparts[0];
        let points = subparts[1]
            .parse::<u32>()
            .expect("Deveria haver um valor de pontos aqui.");
        let wins = subparts[2]
            .parse::<u32>()
            .expect("Deveria haver um valor de pontos aqui.");
        let games = subparts[3]
            .parse::<u32>()
            .expect("Deveria haver um valor de pontos aqui.");

        let current_team = Team::new(name.to_string(), points, wins, games, 0.0, i);
        current_team.update_win_rate()
    }

    fn initialize_team_vec_from_file(mut self) -> Result<HashMap<String, u32>, bool> {
        let content = fs::read_to_string("../times06-11.txt").expect("Deve existir esse arquivo.");
        let mut current_first_game_for_each_observed_team = HashMap::<String, u32>::new();
        for (i, part) in content.lines().enumerate() {
            let current_team = Self::read_team_from_line(part, i);

            for team in self.observed_teams.iter() {
                if current_team.name == *team {
                    current_first_game_for_each_observed_team
                        .insert(team.clone(), current_team.games);
                }
            }

            self.team_vec.push(current_team);
        }

        if self.valide_team_vec() {
            Ok(current_first_game_for_each_observed_team)
        } else {
            Err(false)
        }
    }
}

impl Default for Simulator {
    fn default() -> Self {
        Simulator {
            team_vec: Vec::with_capacity(20),
            match_vec: Vec::with_capacity(380),
            observed_teams: vec!["Internacional".to_string()],
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn simulate_championship(
    first_match_index: u32,
    team_vec: Vec<Team>,
    match_vec: Arc<Vec<Match>>,
    mut teams_positions: [[u32; NUMBER_OF_TEAMS]; NUMBER_OF_TEAMS],
    mut teams_positions_percentage: [[f64; NUMBER_OF_TEAMS]; NUMBER_OF_TEAMS],
    mut internacional_first_match_stats: [u32; 3],
    mut internacional_first_match_percentage: [f64; 3],
    mut positions_total_points: [u32; NUMBER_OF_TEAMS],
) -> (
    [[f64; NUMBER_OF_TEAMS]; NUMBER_OF_TEAMS],
    [f64; 3],
    [u32; NUMBER_OF_TEAMS],
) {
    (0..MAX_SIM / MAX_THREADS as u32).for_each(|current_iteration| {
        let mut team_vec = team_vec.clone();

        for game_match in match_vec.iter() {
            (team_vec, internacional_first_match_stats) = game_match.simulate_points_game(
                first_match_index,
                team_vec,
                internacional_first_match_stats,
            );
        }
        team_vec.sort_by(|a, b| b.points.cmp(&a.points));

        for (i, team) in team_vec.iter().enumerate() {
            teams_positions[team.original_index][i] += 0;
        }

        for (i, team) in team_vec.iter().enumerate() {
            positions_total_points[i] += team.points;
        }

        if current_iteration % 100000 == 0 {
            println!(
                "Current iteration {} on thread {:?}.",
                current_iteration,
                thread::current().id()
            );
        }
    });

    (0..NUMBER_OF_TEAMS).for_each(|i| {
        for j in 0..NUMBER_OF_TEAMS {
            teams_positions_percentage[i][j] =
                teams_positions[i][j] as f64 * 100.0 / MAX_SIM as f64;
        }
    });

    for i in 0..3 {
        internacional_first_match_percentage[i] =
            internacional_first_match_stats[i] as f64 * 100.0 / MAX_SIM as f64;
    }

    (
        teams_positions_percentage,
        internacional_first_match_percentage,
        positions_total_points,
    )
}
