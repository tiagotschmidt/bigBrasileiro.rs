use brasileirao_simulator::display_results::{
    display_header_result, print_teams_full_log, print_teams_summary_log, save_teams_full_log,
    save_teams_summary_log,
};
use brasileirao_simulator::game_match::initialize_match_vec;
use brasileirao_simulator::team::initialize_team_vec;
use brasileirao_simulator::{game_match::Match, team::Team};

use std::sync::Arc;
use std::vec;
use std::{env, thread};

const MAX_SIM: u32 = 1_000_000_000;
const MAX_THREADS: usize = 32;
const NUMBER_OF_TEAMS: usize = 20;

fn main() {
    let args: Vec<String> = env::args().collect();

    let match_vec = Arc::new(initialize_match_vec());
    let (first_match_index, team_vec) = match initialize_team_vec() {
        Ok(return_value) => return_value,
        Err(_) => panic!("Os times possuem pontos, vitórias e jogos incoerentes."),
    };

    let team_vec_for_display = team_vec.clone();

    let all_internacional_first_match_stats = [[0; 3]; MAX_THREADS];
    let mut all_internacional_first_match_percentage = [[0.0; 3]; MAX_THREADS];

    let all_teams_positions = [[[0; NUMBER_OF_TEAMS]; NUMBER_OF_TEAMS]; MAX_THREADS];
    let mut all_teams_positions_percentage =
        [[[0.0; NUMBER_OF_TEAMS]; NUMBER_OF_TEAMS]; MAX_THREADS];
    let mut all_positions_total_points = [[0; NUMBER_OF_TEAMS]; MAX_THREADS];
    let final_percentages = [[0.0; NUMBER_OF_TEAMS]; NUMBER_OF_TEAMS];

    let mut thread_vec = vec![];

    for i in 0..MAX_THREADS {
        let team_vec = team_vec.clone();
        let match_vec = Arc::clone(&match_vec);
        thread_vec.push(thread::spawn(move || {
            {
                simulate_championship(
                    first_match_index,
                    team_vec,
                    match_vec,
                    all_teams_positions[i],
                    all_teams_positions_percentage[i],
                    all_internacional_first_match_stats[i],
                    all_internacional_first_match_percentage[i],
                    all_positions_total_points[i],
                )
            }
        }));
    }

    for (i, handle) in thread_vec.into_iter().enumerate() {
        (
            all_teams_positions_percentage[i],
            all_internacional_first_match_percentage[i],
            all_positions_total_points[i],
        ) = handle.join().unwrap();
    }

    let (final_internacional_first_match_percentage, final_percentages, final_average_points) =
        accumulate_all_threads_results(
            final_percentages,
            all_teams_positions_percentage,
            all_internacional_first_match_percentage,
            all_positions_total_points,
        );

    display_header_result(
        final_internacional_first_match_percentage,
        MAX_SIM,
        MAX_THREADS,
    );

    let is_running_on_github = args.get(1);

    if let Some(boolean_string) = is_running_on_github {
        match boolean_string == &"true".to_string() {
            true => {
                print_teams_full_log(team_vec_for_display.clone(), final_percentages);
                print_teams_summary_log(
                    team_vec_for_display,
                    final_percentages,
                    final_average_points,
                );
            }
            false => {
                save_teams_full_log(team_vec_for_display.clone(), final_percentages);
                save_teams_summary_log(
                    team_vec_for_display,
                    final_percentages,
                    final_average_points,
                );
            }
        }
    } else {
        panic!("Execução de comando shell incorreto! Exemplo: ./brasileirao-simulator false (is_running_on_github)")
    }
}

fn accumulate_all_threads_results(
    mut final_percentages: [[f64; NUMBER_OF_TEAMS]; NUMBER_OF_TEAMS],
    all_teams_positions_percentage: [[[f64; NUMBER_OF_TEAMS]; NUMBER_OF_TEAMS]; MAX_THREADS],
    all_internacional_first_match_percentage: [[f64; 3]; MAX_THREADS],
    all_positions_total_points: [[u32; NUMBER_OF_TEAMS]; MAX_THREADS],
) -> (
    [f64; 3],
    [[f64; NUMBER_OF_TEAMS]; NUMBER_OF_TEAMS],
    [f64; NUMBER_OF_TEAMS],
) {
    let mut all_positions_average_points = [0.0; NUMBER_OF_TEAMS];

    (0..NUMBER_OF_TEAMS).for_each(|i| {
        (0..MAX_THREADS).for_each(|j| {
            for k in 0..NUMBER_OF_TEAMS {
                final_percentages[i][k] += all_teams_positions_percentage[j][i][k];
            }
        });
    });

    let mut final_internacional_first_match_percentage = [0.0; 3];

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

    (0..NUMBER_OF_TEAMS).for_each(|i| {
        (0..MAX_THREADS).for_each(|j| {
            all_positions_average_points[i] += all_positions_total_points[j][i] as f64;
        });
    });

    (0..NUMBER_OF_TEAMS).for_each(|i| {
        all_positions_average_points[i] /= MAX_SIM as f64;
    });

    (
        final_internacional_first_match_percentage,
        final_percentages,
        all_positions_average_points,
    )
}

#[allow(clippy::too_many_arguments)]
fn simulate_championship(
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
            teams_positions[team.original_index][i] += 1;
        }

        for (i, team) in team_vec.iter().enumerate() {
            positions_total_points[i] += team.points;
        }

        if current_iteration % 10000 == 0 {
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
