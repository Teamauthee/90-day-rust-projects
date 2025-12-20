fn main() {
    let mut game_state = Scoreboard {
        home_team: String::from("Lakers"),
        away_team: String::from("Celtics"),
        home_score: 0,
        away_score: 0,
    };

    update_scoreboard(&mut game_state, true, true);

    let reader = &game_state;
    println!(
        "{} : {}\n{} : {}",
        reader.home_team, reader.home_score, reader.away_team, reader.away_score
    );
}

struct Scoreboard {
    home_team: String,
    away_team: String,
    home_score: u32,
    away_score: u32,
}

fn update_scoreboard(sb: &mut Scoreboard, is_home: bool, is_3pts: bool) -> u32 {
    let points = if is_3pts { 3 } else { 2 };
    if is_home {
        sb.home_score += points;
    } else {
        sb.away_score += points;
    }
    points
}
