fn main() {
    let games = vec![
        Game {
            home_team: String::from("Young Boys"),
            away_team: String::from("Basel"),
            home_score: 1.0,
            away_score: 1.0,
        },
        Game {
            home_team: String::from("Servette FC"),
            away_team: String::from("FC Zurich"),
            home_score: 2.0,
            away_score: 0.0,
        },
    ];

    let avg: f32 = avg_goals(&games);
    let involved: bool = was_team_invovled(&games, "Basel");
    println!("Average goals: {}", avg);
    println!("Was Basel involved: {}", involved);
}

struct Game {
    home_team: String,
    away_team: String,
    home_score: f32,
    away_score: f32,
}

fn total_games(games: &Vec<Game>) -> usize {
    games.len()
}

fn avg_goals(games: &Vec<Game>) -> f32 {
    let total_games = total_games(games);
    let mut total_goals: f32 = 0.0;
    for game in games {
        total_goals += game.home_score + game.away_score;
    }
    total_goals / total_games as f32
}

fn was_team_invovled(games: &Vec<Game>, team: &str) -> bool {
    for game in games {
        if game.home_team == team || game.away_team == team {
            return true;
        }
    }
    false
}
