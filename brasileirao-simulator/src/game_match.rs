use crate::team::Team;

#[derive(Debug)]
pub struct Match<'a> {
    pub first_team: &'a Team,
    pub second_team: &'a Team,
}

impl<'a> Match<'a> {
    pub fn new(match_string: &str, teams_vec: &'a [Team]) -> Self {
        let subparts = match_string.split(' ').collect::<Vec<_>>();
        let first_team = teams_vec
            .iter()
            .find(|item| item.name == subparts[2])
            .unwrap();
        let second_team = teams_vec
            .iter()
            .find(|item| item.name == subparts[2])
            .unwrap();

        Match {
            first_team,
            second_team,
        }
    }

    pub fn simulate_points_game(self, teams_array: [Team; 40]) -> [Team; 40] {
        todo!()
    }
}
