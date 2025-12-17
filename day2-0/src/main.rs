use std::io;

fn main() {
    println!("Give a temperature value in Celsius");

    let celsius: f32 = loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("That is not a valid number, try again!");
                continue;
            }
        };
    };

    let fahrenheit = (celsius * 1.8) + 32.0;

    println!("The temperature in Fahrenheit is: {}", fahrenheit)
}
