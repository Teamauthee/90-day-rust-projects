fn main() {
    let team_a: String = String::from("Los Angeles Clippers");
    let team_b: String = String::from("Los Angeles Lakers");
    let result = longest_name(&team_a, &team_b);

    println!("The longest name is {result}");
}

fn longest_name<'a>(name1: &'a str, name2: &'a str) -> &'a str {
    if name1.len() > name2.len() {
        name1
    } else {
        name2
    }
}
