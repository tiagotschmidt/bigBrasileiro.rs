use std::fmt::Display;
use std::fs;

#[derive(Clone, Debug)]
pub struct Team {
    pub name: String,
    pub points: u32,
    pub wins: u32,
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

    pub fn update_win_rate(self) -> Self {
        let mut updated_team = self.clone();
        updated_team.win_rate = self.wins as f32 / self.games as f32;
        updated_team
    }

    pub fn win(self) -> Self {
        let mut updated_team = self.clone();
        updated_team.games += 1;
        updated_team.wins += 1;
        updated_team.update_win_rate()
    }

    pub fn lose(self) -> Self {
        let mut updated_team = self.clone();
        updated_team.games += 1;
        updated_team.update_win_rate()
    }

    pub fn win_points(self) -> Self {
        let mut updated_team = self.clone();
        updated_team = updated_team.win();
        updated_team.points += 3;
        updated_team.update_win_rate()
    }

    pub fn tie_points(self) -> Self {
        let mut updated_team = self.clone();
        updated_team.games += 1;
        updated_team.points += 1;
        updated_team.update_win_rate()
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

pub fn read_team_content(primary_path: &str) -> String {
    if let Ok(c) = fs::read_to_string(primary_path) {
        return c;
    }
    let parent_path = format!("../{}", primary_path);
    if let Ok(c) = fs::read_to_string(&parent_path) {
        return c;
    }
    if let Ok(c) = fs::read_to_string("times2026.txt") {
        return c;
    }
    if let Ok(c) = fs::read_to_string("../times2026.txt") {
        return c;
    }
    if let Ok(c) = fs::read_to_string("times06-11.txt") {
        return c;
    }
    if let Ok(c) = fs::read_to_string("../times06-11.txt") {
        return c;
    }
    panic!("Unable to find teams file: {}", primary_path);
}

pub fn initialize_team_vec() -> Result<(u32, Vec<Team>), bool> {
    initialize_team_vec_from_file("times2026.txt")
}

pub fn initialize_team_vec_from_file(path: &str) -> Result<(u32, Vec<Team>), bool> {
    let mut team_vec: Vec<Team> = Vec::with_capacity(20);
    let content = read_team_content(path);
    let mut current_first_game = 0;
    for (i, part) in content.lines().enumerate() {
        if part.trim().is_empty() {
            continue;
        }
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
    let file_input_points = team.points;

    let current_ties_and_loses = team.games - team.wins;

    let sum_of_all_possible_points = team.wins * 3 + current_ties_and_loses;

    file_input_points <= sum_of_all_possible_points //This happens because our model does not
                                                    //differentiate lose and tie. Therefore, our
                                                    //right hand estimative is always the input
                                                    //amount or more (considering all not wind ==
                                                    //ties).
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_team_parsing() {
        let team = read_team_from_line("Flamengo;7;2;3", 0);
        assert_eq!(team.name, "Flamengo");
        assert_eq!(team.points, 7);
        assert_eq!(team.wins, 2);
        assert_eq!(team.games, 3);
        assert!((team.win_rate - 2.0 / 3.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_assert_points_valid() {
        // 2 wins * 3 = 6 pts + 1 tie = 7 pts in 3 games -> valid
        let team = Team::new("Flamengo".to_string(), 7, 2, 3, 0.66, 0);
        assert!(assert_points(&team));
    }

    #[test]
    fn test_assert_points_invalid() {
        // 2 wins * 3 = 6 pts max from wins, + 0 non-wins = 6 pts max in 2 games, but claims 7 points -> invalid
        let team = Team::new("Incoherent".to_string(), 7, 2, 2, 1.0, 0);
        assert!(!assert_points(&team));
    }
}

