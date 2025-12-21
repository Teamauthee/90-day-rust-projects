fn main() {
    let master_buffer = String::from("3pts, Lebron James, Los Angeles Lakers");

    let views: Vec<Gameview> = master_buffer
        .lines()
        .map(|line| parse_to_view(line))
        .collect();

    for view in views {
        println!("{}", view.player_and_team());
        println!("{}", view.get_score_type());
    }
}

struct Gameview<'a> {
    score_type: &'a str,
    player_name: &'a str,
    team_name: &'a str,
}

fn parse_to_view<'a>(data: &'a str) -> Gameview<'a> {
    let mut parts = data.split(",").map(|s| s.trim());

    Gameview {
        score_type: parts.next().unwrap_or("N/A"),
        player_name: parts.next().unwrap_or("Unknown"),
        team_name: parts.next().unwrap_or("Unknown"),
    }
}

impl<'a> Gameview<'a> {
    fn player_and_team(&self) -> String {
        format!("{} playing for {}", self.player_name, self.team_name)
    }
    fn get_score_type(&self) -> &'a str {
        self.score_type
    }
}
