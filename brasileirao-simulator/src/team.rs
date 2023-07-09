use std::fs;

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

pub fn initialize_team_vec() -> Vec<Team> {
    let mut team_vec: Vec<Team> = Vec::with_capacity(20);
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

        let mut current_team = Team::new(name.to_string(), points, wins, games, 0.0, i);
        current_team = current_team.update_win_rate();
        team_vec.push(current_team);
    }
    team_vec
}
