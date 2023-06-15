use std::fs;

use brasileirao_simulator::{game_match::Match, team::Team};
fn main() {
    let mut team_vec = initialize_team_vec();
    let mut match_vec = initialize_match_vec();

    println!("Vetor dos times: {:#?}", team_vec.clone());
    team_vec = match_vec[0].simulate_points_game(team_vec);    // for game_match in match_vec {
    //     println!("Partida atual: {:#?}", game_match);
    //     team_vec = game_match.simulate_points_game(team_vec);
    // }
    println!("Vetor dos times: {:#?}", team_vec);
}

fn initialize_match_vec() -> Vec<Match> {
    let mut match_vec: Vec<Match> = Vec::new();
    let content = fs::read_to_string("../jogos13-06.txt").expect("Deve existir esse arquivo.");
    for part in content.lines() {
        let current_match = Match::new(part);
        match_vec.push(current_match);
    }
    match_vec
}

fn initialize_team_vec() -> [Team; 20] {
    let mut team_vec: [Team; 20];
    let content = fs::read_to_string("../times13-06.txt").expect("Deve existir esse arquivo.");
    for (i, part) in content.lines().enumerate() {
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
        team_vec[i] = current_team;
    }
    team_vec
}
