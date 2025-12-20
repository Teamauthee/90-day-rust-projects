fn main() {
    let games = vec![
        Game {
            home_team: String::from("Lakers"),
            away_team: String::from("Celtics"),
            home_score: 132,
            away_score: 131,
        },
        Game {
            home_team: String::from("Warriors"),
            away_team: String::from("Lakers"),
            home_score: 108,
            away_score: 93,
        },
        Game {
            home_team: String::from("Celtics"),
            away_team: String::from("Warriors"),
            home_score: 110,
            away_score: 121,
        },
    ];
    let biggest_game = largest_win(&games);
    println!(
        "The largest win of all games is {} vs {}",
        biggest_game.home_team, biggest_game.away_team
    );
}

struct Game {
    home_team: String,
    away_team: String,
    home_score: i32,
    away_score: i32,
}

fn largest_win(games: &[Game]) -> &Game {
    let mut biggest_gap: i32 = 0;
    let mut best_game = &games[0];

    for game in games {
        let gap = game.home_score - game.away_score;
        if gap > biggest_gap {
            biggest_gap = gap;
            best_game = game;
        }
    }
    best_game
}
