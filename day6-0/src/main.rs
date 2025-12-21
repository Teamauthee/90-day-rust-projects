fn main() {
    let action = String::from("3pts, Lebron James, Los Angeles Lakers");
    let team = String::from("Los Angeles Lakers");

    let comments = find_team(&action, &team);

    let action_event = get_event(&action);

    let player = get_player(&action);

    println!("The commentary is {}", comments);
    println!("The event is a {action_event} shot");
    println!("The scorer is {player}");
}

fn find_team<'a, 'b>(commentary: &'a str, team_name: &'b str) -> &'a str {
    if commentary.contains(team_name) {
        commentary
    } else {
        "No team found"
    }
}

fn get_event<'a>(commentary: &'a str) -> &'a str {
    let commentary: &str = commentary
        .split(",")
        .next()
        .map(|s| s.trim())
        .unwrap_or("Unknown event");
    commentary
}

fn get_player<'a>(s: &'a str) -> &'a str {
    s.split(',').nth(1).map(|p| p.trim()).unwrap_or("N/A")
}
