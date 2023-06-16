use brasileirao_simulator::{game_match::Match, team::Team};
use std::fs;

const MAX_SIM: i32 = 1000000;

fn main() {
    let mut internacional_positions = [0; 20];
    let match_vec = initialize_match_vec();
    let team_vec = initialize_team_vec();
    for _i in 0..MAX_SIM {
        let mut team_vec = team_vec.clone();
        for game_match in match_vec.clone() {
            team_vec = game_match.simulate_points_game(team_vec);
        }

        team_vec.sort_by(|a, b| b.points.cmp(&a.points));

        let mut team_iter = team_vec.iter();
        let mut team = team_iter.next().unwrap();
        let mut counter = 0;
        let internacional_current_position = loop {
            if team.name == "Internacional" {
                break counter;
            }
            counter += 1;
            team = team_iter.next().unwrap();
        };

        //println!("Vetor dos times2: {:#?}", team_vec);
        internacional_positions[internacional_current_position] += 1;
    }

    println!("Array do inter: {:#?}", internacional_positions);
}

fn initialize_match_vec() -> Vec<Match> {
    let mut match_vec: Vec<Match> = Vec::with_capacity(380);
    let content = fs::read_to_string("../jogos13-06.txt").expect("Deve existir esse arquivo.");
    for part in content.lines() {
        let current_match = Match::new(part);
        match_vec.push(current_match);
    }
    match_vec
}

fn initialize_team_vec() -> Vec<Team> {
    let mut team_vec: Vec<Team> = Vec::with_capacity(20);
    let content = fs::read_to_string("../times13-06.txt").expect("Deve existir esse arquivo.");
    for part in content.lines() {
        let subparts = part.split('>').collect::<Vec<_>>();
        let name = subparts[78].split('<').next().unwrap();
        let points: u32 = subparts[81]
            .split('<')
            .next()
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let wins: u32 = subparts[85]
            .split('<')
            .next()
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let games: u32 = subparts[83]
            .split('<')
            .next()
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let mut current_team = Team::new(name.to_string(), points, wins, games, 0.0);
        current_team = current_team.update_win_rate();
        team_vec.push(current_team);
    }
    team_vec
}
