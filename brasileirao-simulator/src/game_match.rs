use crate::team::Team;

#[derive(Debug)]
pub struct Match<'a> {
    first_team: &'a Team,
    second_team: &'a Team,
}

impl<'a> Match<'a> {
    pub fn new(match_string: String, teams_array: &'a [Team; 40]) -> Self {
        let i = 0;
        let first_team = loop {
            if teams_array[i].name == "foo" {
                break &teams_array[i];
            }
        };
        let second_team = loop {
            if teams_array[i].name == "bar" {
                break &teams_array[i];
            }
        };

        Match {
            first_team,
            second_team,
        }
    }

    pub fn simulate_points_game(self, teams_array: [Team; 40]) -> [Team; 40] {
        todo!()
    }
}
