use std::fmt;

#[derive(Debug)]
struct Player {
    name: String,
    score: u32,
}

impl Player {
    fn new(name: &str, score: u32) -> Self {
        Self {
            name: String::from(name),
            score,
        }
    }
}

#[derive(Debug)]
struct Team {
    name: String,
    roster: Vec<Player>,
}

impl Team {
    fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            roster: Vec::new(),
        }
    }
    fn add_player(&mut self, player: Player) {
        self.roster.push(player)
    }
    fn remove_player(&mut self, target_name: &str) -> Option<Player> {
        if let Some(index) = self.roster.iter().position(|p| p.name == target_name) {
            Some(self.roster.remove(index))
        } else {
            None
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({} pts)", self.name, self.score)
    }
}

fn main() {
    let mut my_team = Team::new("Los Angeles Lakers");
    let p1 = Player::new("Lebron James", 40);

    println!("Created player: {}", p1);

    my_team.add_player(p1);
    println!("{:?}", my_team);

    let removed_player = my_team.remove_player("Lebron James");

    match removed_player {
        Some(p) => println!("Transferred out: {}", p),
        None => println!("Player not found"),
    }
    println!("Team after remove: {:?}", my_team);
}
