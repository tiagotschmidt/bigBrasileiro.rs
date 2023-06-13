use std::fs;

use brasileirao_simulator::team::Team;
fn main() {
    let content = fs::read_to_string("tiago.txt").expect("Deve existir esse arquivo.");
    let contents = content.split('\n');

    let mut team_vec: Vec<Team> = Vec::new();

    for (i, part) in contents.enumerate() {
        let mut name = "";
        let mut points: u32 = 0;
        let mut wins: u32 = 0;
        let mut games: u32 = 0;

        println!("##################################");

        if i % 8 == 0 {
            let subparts: Vec<&str> = part.split('>').collect();
            let testes = part.split('>').enumerate();
            for teste in testes {
                println!("{:#?}", teste);
            }
            name = subparts[11].split('<').next().unwrap();
            points = subparts[14]
                .split('<')
                .next()
                .unwrap()
                .parse::<u32>()
                .unwrap();
            wins = subparts[18]
                .split('<')
                .next()
                .unwrap()
                .parse::<u32>()
                .unwrap();
            games = subparts[16]
                .split('<')
                .next()
                .unwrap()
                .parse::<u32>()
                .unwrap();
            // for (j, subpart) in part.split('>').enumerate() {
            //     println!("Loucura:{:#?}", subpart);
            //     if j == 11 {
            //         name = subpart.split('<').next().unwrap();
            //     };
            //     if j == 14 {
            //         points = subpart.split('<').next().unwrap().parse::<u32>().unwrap();
            //     };
            //     if j == 18 {
            //         wins = subpart.split('<').next().unwrap().parse::<u32>().unwrap();
            //     };
            //     if j == 16 {
            //         games = subpart.split('<').next().unwrap().parse::<u32>().unwrap();
            //     };
            // }
        }
        let current_team = Team::new(name.to_string(), points, wins, games, 0.0);
        team_vec.push(current_team);
    }
    //println!("Time atual: {:#?}", team_vec);
}
