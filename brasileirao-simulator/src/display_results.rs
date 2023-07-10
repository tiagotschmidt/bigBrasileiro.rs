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

pub fn display_summary_result(team_name: String, internacional_positions_percentage: [f64; 20]) {
    println!("{}", team_name);
    println!(
        "Chances de ser Campeão:     \t{:.2}%.",
        internacional_positions_percentage[0]
    );
    println!(
        "Chances de ser Vice Campeão:\t{:.2}%.",
        internacional_positions_percentage[1]
    );
    println!(
        "Chances de ser Rebaixado:   \t{:.2}%.",
        internacional_positions_percentage[19]
            + internacional_positions_percentage[18]
            + internacional_positions_percentage[17]
            + internacional_positions_percentage[16]
    );
}
