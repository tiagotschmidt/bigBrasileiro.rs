use brasileirao_simulator::{game_match::Match, team::Team};

use std::fs;
use std::thread;
use std::vec;

const MAX_SIM: u32 = 1_000_000;
const MAX_THREADS: usize = 8;
const MAX_TEAMS: usize = 20;

fn main() {
    let all_internacional_positions = [[0; MAX_TEAMS]; MAX_THREADS];
    let mut all_internacional_positions_percentage = [[0.0; MAX_TEAMS]; MAX_THREADS];
    let mut final_internacional_percentage = [0.0; MAX_TEAMS];
    let match_vec = initialize_match_vec();
    let team_vec = initialize_team_vec();

    let mut thread_vec = vec![];

    for i in 0..MAX_THREADS {
        let team_vec = team_vec.clone();
        let match_vec = match_vec.clone();
        thread_vec.push(thread::spawn(move || {
            {
                simulate_championship(
                    team_vec,
                    match_vec,
                    all_internacional_positions[i],
                    all_internacional_positions_percentage[i],
                )
            }
        }));
    }

    for (i, handle) in thread_vec.into_iter().enumerate() {
        all_internacional_positions_percentage[i] = handle.join().unwrap();
    }

    for (i, _item) in final_internacional_percentage.iter_mut().enumerate() {
        let mut acc = 0.0;

        for item in all_internacional_positions_percentage.iter() {
            acc += item[i];
        }

        *_item = acc;
    }

    display_result_for_inter("Internacional".to_string(), final_internacional_percentage);
}

fn simulate_championship(
    team_vec: Vec<Team>,
    match_vec: Vec<Match>,
    mut internacional_positions: [i32; 20],
    mut internacional_positions_percentage: [f64; 20],
) -> [f64; 20] {
    for _i in 0..MAX_SIM / MAX_THREADS as u32 {
        let mut team_vec = team_vec.clone();

        for game_match in match_vec.clone() {
            team_vec = game_match.simulate_points_game(team_vec);
        }
        team_vec.sort_by(|a, b| b.points.cmp(&a.points));

        let internacional_current_position =
            search_team_placement("Internacional".to_string(), team_vec);
        internacional_positions[internacional_current_position] += 1;
    }

    for i in 0..MAX_TEAMS {
        internacional_positions_percentage[i] =
            internacional_positions[i] as f64 * 100.0 / MAX_SIM as f64;
    }
    internacional_positions_percentage
}

fn search_team_placement(team_name: String, team_vec: Vec<Team>) -> usize {
    let mut team_iter = team_vec.iter();
    let mut team = team_iter.next().unwrap();
    let mut counter = 0;

    loop {
        if team.name == team_name {
            break counter;
        }
        counter += 1;
        team = team_iter.next().unwrap();
    }
}

fn display_result_for_inter(team_name: String, internacional_positions_percentage: [f64; 20]) {
    println!(
        "###################\tResumo de Simulação com {} repetições e {} threads\t###################",
        MAX_SIM, MAX_THREADS
    );
    println!("{}", team_name);
    println!(
        "Chances de ser Campeão:     \t{}%.",
        internacional_positions_percentage[0]
    );
    println!(
        "Chances de ser Vice Campeão:\t{}%.",
        internacional_positions_percentage[1]
    );
    println!(
        "Chances de ser Rebaixado:   \t{}%.",
        internacional_positions_percentage[19]
            + internacional_positions_percentage[18]
            + internacional_positions_percentage[17]
            + internacional_positions_percentage[16]
    );
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
