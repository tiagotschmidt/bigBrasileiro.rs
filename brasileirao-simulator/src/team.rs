//use std::io::Write;
use std::{
    fmt::Display,
    fs::{self},
};

#[derive(Clone, Debug)]
pub struct Team {
    pub name: String,
    pub points: u32,
    wins: u32,
    pub games: u32,
    pub win_rate: f32,
    pub original_index: usize,
}

impl Team {
    pub fn new(
        name: String,
        points: u32,
        wins: u32,
        games: u32,
        win_rate: f32,
        original_index: usize,
    ) -> Self {
        Team {
            name,
            points,
            wins,
            games,
            win_rate,
            original_index,
        }
    }

    pub fn update_win_rate(mut self) -> Self {
        self.win_rate = self.wins as f32 / self.games as f32;
        self
    }

    pub fn win(mut self) -> Self {
        self.games += 1;
        self.wins += 1;
        self.update_win_rate()
    }

    pub fn lose(mut self) -> Self {
        self.games += 1;
        self = self.update_win_rate();
        self
    }

    pub fn win_points(mut self) -> Self {
        self = self.win();
        self.points += 3;
        self.update_win_rate()
    }

    pub fn tie_points(mut self) -> Self {
        self.games += 1;
        self.points += 1;
        self.update_win_rate()
    }
}

impl Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{};{};{};{}",
            self.name, self.points, self.wins, self.games
        )
    }
}

impl Default for Team {
    fn default() -> Self {
        Team {
            name: "".to_string(),
            points: 0,
            wins: 0,
            games: 0,
            win_rate: 0.00,
            original_index: 0,
        }
    }
}

pub fn initialize_team_vec() -> Result<(u32, Vec<Team>), bool> {
    let mut team_vec: Vec<Team> = Vec::with_capacity(20);
    let content = fs::read_to_string("../times01-09.txt").expect("Deve existir esse arquivo.");
    let mut current_first_game = 0;
    for (i, part) in content.lines().enumerate() {
        let current_team = read_team_from_line(part, i);

        if current_team.name == "Internacional" {
            current_first_game = current_team.games;
        }
        team_vec.push(current_team);
    }

    if team_vec.iter().all(assert_points) {
        Ok((current_first_game, team_vec))
    } else {
        Err(false)
    }
}

fn assert_points(team: &Team) -> bool {
    let expected_points = team.points;

    let current_ties = team.games - team.wins;

    let current_points = team.wins * 3 + current_ties;

    expected_points <= current_points
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

    let mut current_team = Team::new(name.to_string(), points, wins, games, 0.0, i);
    current_team = current_team.update_win_rate();
    current_team
}

//#[test]
//fn test() {
//    let (_first_game, mut team_vec) = match initialize_team_vec() {
//        Ok(team_vec) => team_vec,
//        Err(_) => panic!("Os times possuem pontos, vitórias e jogos incoerentes."),
//    };
//
//    let output_file =
//        File::create("jogos31-07_new.txt").expect("Não foi possível criar o arquivo.");
//
//    team_vec.sort_by(|a, b| b.points.cmp(&a.points));
//
//    for team in team_vec {
//        writeln!(&output_file, "{}", team).unwrap();
//    }
//}
