use crate::team::Team;
use chrono::{DateTime, Local};
use std::{fs::File, io::Write};

pub fn display_header_result(
    final_internacional_first_match_percentage: [f64; 3],
    max_sim: u32,
    max_threads: usize,
) {
    println!(
        "###################\tResumo de Simulação com {} repetições e {} threads\t###################",
        max_sim, max_threads
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

pub fn display_summary_result(team_name: String, positions_percentage: [f64; 20]) {
    println!("{}", team_name);
    println!(
        "Chances de ser Campeão:     \t{:.2}%.",
        positions_percentage[0]
    );
    println!(
        "Chances de ser Vice Campeão:\t{:.2}%.",
        positions_percentage[1]
    );
    println!(
        "Chances de ser Rebaixado:   \t{:.2}%.",
        positions_percentage[19]
            + positions_percentage[18]
            + positions_percentage[17]
            + positions_percentage[16]
    );
}

pub fn save_teams_full_log(team_vec_for_display: Vec<Team>, final_positions: [[f64; 20]; 20]) {
    let now: DateTime<Local> = Local::now();
    let now_str = format!("logs/full_table_{}", now);
    let output_file = File::create(&now_str).expect("Não foi possível criar o arquivo.");

    for team in team_vec_for_display.iter() {
        writeln!(&output_file, "Time:{}", team.name).unwrap();
        for (i, percentage) in final_positions[team.original_index].iter().enumerate() {
            writeln!(&output_file, "Posição {}: {}%", i + 1, percentage).unwrap();
        }
        writeln!(&output_file).unwrap();
    }
}

pub fn print_teams_full_log(team_vec_for_display: Vec<Team>, final_positions: [[f64; 20]; 20]) {
    for team in team_vec_for_display.iter() {
        println!("Time:{}", team.name);
        for (i, percentage) in final_positions[team.original_index].iter().enumerate() {
            println!("Posição {}: {}%", i + 1, percentage);
        }
        println!();
    }
}

pub fn save_teams_summary_log(team_vec_for_display: Vec<Team>, final_positions: [[f64; 20]; 20]) {
    let now: DateTime<Local> = Local::now();
    let now_str = format!("logs/summary_table_{}", now);
    let output_file = File::create(&now_str).expect("Não foi possível criar o arquivo.");

    for team in team_vec_for_display.iter() {
        generate_teams_summary_log_inner(
            output_file.try_clone().unwrap(),
            team.name.to_string(),
            final_positions[team.original_index],
        );
    }
}

pub fn print_teams_summary_log(team_vec_for_display: Vec<Team>, final_positions: [[f64; 20]; 20]) {
    for team in team_vec_for_display.iter() {
        println!("{}", team.name);
        println!(
            "Chances de ser Campeão:     \t{:.2}%.",
            final_positions[team.original_index][0]
        );
        println!(
            "Chances de ser Vice Campeão:\t{:.2}%.",
            final_positions[team.original_index][1]
        );
        println!(
            "Chances de ser Rebaixado:   \t{:.2}%.",
            final_positions[team.original_index][19]
                + final_positions[team.original_index][18]
                + final_positions[team.original_index][17]
                + final_positions[team.original_index][16]
        );
    }
}

pub fn generate_teams_summary_log_inner(
    output_file: File,
    team_name: String,
    positions_percentage: [f64; 20],
) {
    writeln!(&output_file, "{}", team_name).unwrap();
    writeln!(
        &output_file,
        "Chances de ser Campeão:     \t{:.2}%.",
        positions_percentage[0]
    )
    .unwrap();
    writeln!(
        &output_file,
        "Chances de ser Vice Campeão:\t{:.2}%.",
        positions_percentage[1]
    )
    .unwrap();
    writeln!(
        &output_file,
        "Chances de ser Rebaixado:   \t{:.2}%.",
        positions_percentage[19]
            + positions_percentage[18]
            + positions_percentage[17]
            + positions_percentage[16]
    )
    .unwrap();
}
