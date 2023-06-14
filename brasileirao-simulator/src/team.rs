#[derive(Clone, Debug)]
pub struct Team {
    pub name: String,
    points: u32,
    wins: u32,
    games: u32,
    pub win_rate: f32,
}

impl Team {
    pub fn new(name: String, points: u32, wins: u32, games: u32, win_rate: f32) -> Self {
        Team {
            name,
            points,
            wins,
            games,
            win_rate,
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
