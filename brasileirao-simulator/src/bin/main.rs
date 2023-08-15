use brasileirao_simulator::display_results::{
    display_header_result, print_teams_full_log, print_teams_summary_log, save_teams_full_log,
    save_teams_summary_log,
};
use brasileirao_simulator::game_match::initialize_match_vec;
use brasileirao_simulator::team::initialize_team_vec;
use brasileirao_simulator::{game_match::Match, team::Team};

use std::vec;
use std::{env, thread};

const MAX_SIM: u32 = 100;
const MAX_THREADS: usize = 16;
const MAX_TEAMS: usize = 20;

fn main() {
    let args: Vec<String> = env::args().collect();

    let match_vec = initialize_match_vec();
    let (first_match_index, team_vec) = match initialize_team_vec() {
        Ok(team_vec) => team_vec,
        Err(_) => panic!("Os times possuem pontos, vitórias e jogos incoerentes."),
    };

    let (_, team_vec_for_display) = match initialize_team_vec() {
        Ok(team_vec) => team_vec,
        Err(_) => panic!("Os times possuem pontos, vitórias e jogos incoerentes."),
    };

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
                    first_match_index,
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

    display_header_result(
        *final_internacional_first_match_percentage,
        MAX_SIM,
        MAX_THREADS,
    );

    let is_running_on_github = args.get(1);

    if let Some(boolean_string) = is_running_on_github {
        match boolean_string == &"true".to_string() {
            true => {
                print_teams_full_log(team_vec_for_display.clone(), final_percentages);
                print_teams_summary_log(team_vec_for_display, final_percentages);
            }
            false => {
                save_teams_full_log(team_vec_for_display.clone(), final_percentages);
                save_teams_summary_log(team_vec_for_display, final_percentages);
            }
        }
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
    first_match_index: u32,
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
