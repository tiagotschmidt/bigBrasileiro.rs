use brasileirao_simulator::game_match::initialize_match_vec;
use brasileirao_simulator::team::initialize_team_vec;
use brasileirao_simulator::{game_match::Match, team::Team};

use std::thread;
use std::vec;

const MAX_SIM: u32 = 1_000_000;
const MAX_THREADS: usize = 16;
const MAX_TEAMS: usize = 20;

fn main() {
    let match_vec = initialize_match_vec();
    let team_vec = initialize_team_vec();
    let team_vec_for_display = initialize_team_vec();

    let all_internacional_first_match_stats = [[0; 3]; MAX_THREADS];
    let mut all_internacional_first_match_percentage = [[0.0; 3]; MAX_THREADS];
    let mut final_internacional_first_match_percentage = [0.0; 3];

    let all_teams_positions = [[[0; MAX_TEAMS]; MAX_TEAMS]; MAX_THREADS];
    let mut all_teams_positions_percentage = [[[0.0; MAX_TEAMS]; MAX_TEAMS]; MAX_THREADS];
    let final_percentages = [[0.0; MAX_TEAMS]; MAX_TEAMS];

    let mut thread_vec = vec![];

    for i in 0..MAX_THREADS {
        let team_vec = team_vec.clone();
        let match_vec = match_vec.clone();
        thread_vec.push(thread::spawn(move || {
            {
                simulate_championship(
                    team_vec,
                    match_vec,
                    all_teams_positions[i],
                    all_teams_positions_percentage[i],
                    all_internacional_first_match_stats[i],
                    all_internacional_first_match_percentage[i],
                )
            }
        }));
    }

    for (i, handle) in thread_vec.into_iter().enumerate() {
        (
            all_teams_positions_percentage[i],
            all_internacional_first_match_percentage[i],
        ) = handle.join().unwrap();
    }

    let (final_internacional_first_match_percentage, final_percentages) =
        accumulate_all_threads_results(
            final_percentages,
            all_teams_positions_percentage,
            &mut final_internacional_first_match_percentage,
            all_internacional_first_match_percentage,
        );

    display_header_result(*final_internacional_first_match_percentage);

    for team in team_vec_for_display.iter() {
        display_result(
            team.name.to_string(),
            final_percentages[team.original_index],
        );
    }
}

fn accumulate_all_threads_results(
    mut final_percentages: [[f64; MAX_TEAMS]; MAX_TEAMS],
    all_teams_positions_percentage: [[[f64; MAX_TEAMS]; MAX_TEAMS]; MAX_THREADS],
    final_internacional_first_match_percentage: &mut [f64; 3],
    all_internacional_first_match_percentage: [[f64; 3]; MAX_THREADS],
) -> (&mut [f64; 3], [[f64; MAX_TEAMS]; MAX_TEAMS]) {
    (0..MAX_TEAMS).for_each(|i| {
        (0..MAX_THREADS).for_each(|j| {
            for k in 0..MAX_TEAMS {
                final_percentages[i][k] += all_teams_positions_percentage[j][i][k];
            }
        });
    });

    for (i, _item) in final_internacional_first_match_percentage
        .iter_mut()
        .enumerate()
    {
        let mut acc = 0.0;

        for item in all_internacional_first_match_percentage.iter() {
            acc += item[i];
        }

        *_item = acc;
    }

    (
        final_internacional_first_match_percentage,
        final_percentages,
    )
}

fn simulate_championship(
    team_vec: Vec<Team>,
    match_vec: Vec<Match>,
    mut teams_positions: [[u32; MAX_TEAMS]; MAX_TEAMS],
    mut teams_positions_percentage: [[f64; MAX_TEAMS]; MAX_TEAMS],
    mut internacional_first_match_stats: [u32; 3],
    mut internacional_first_match_percentage: [f64; 3],
) -> ([[f64; MAX_TEAMS]; MAX_TEAMS], [f64; 3]) {
    for _i in 0..MAX_SIM / MAX_THREADS as u32 {
        let mut team_vec = team_vec.clone();

        for game_match in match_vec.clone() {
            (team_vec, internacional_first_match_stats) =
                game_match.simulate_points_game(team_vec, internacional_first_match_stats);
        }
        team_vec.sort_by(|a, b| b.points.cmp(&a.points));

        for (i, team) in team_vec.iter().enumerate() {
            teams_positions[team.original_index][i] += 1;
        }
    }

    (0..MAX_TEAMS).for_each(|i| {
        for j in 0..MAX_TEAMS {
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
    )
}

fn display_result(team_name: String, internacional_positions_percentage: [f64; 20]) {
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

fn display_header_result(final_internacional_first_match_percentage: [f64; 3]) {
    println!(
        "###################\tResumo de Simulação com {} repetições e {} threads\t###################",
        MAX_SIM, MAX_THREADS
    );

    println!(
        "Chances de vencer a primeira partida:\t{}%.",
        final_internacional_first_match_percentage[0]
    );
    println!(
        "Chances de perder a primeira partida:\t{}%.",
        final_internacional_first_match_percentage[2]
    );
    println!(
        "Chances de empatar a primeira partida:\t{}%.",
        final_internacional_first_match_percentage[1]
    );
}
