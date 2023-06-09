#[derive(Debug)]
pub struct Team {
    pub name: String,
    points: u32,
    wins: u32,
    games: u32,
    win_rate: f32,
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
        self.update_win_rate()
    }

    pub fn win_points(self) -> Self {
        let mut new_self = self.win();
        new_self.points += 3;
        new_self
    }

    pub fn tie_points(self) -> Self {
        let mut new_self = self.lose();
        new_self.points += 1;
        new_self
    }
}
