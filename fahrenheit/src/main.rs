use std::io;

fn main() {

    println!("Enter the temperature in Celsius: ");
    let mut input = String::new();

    io::stdin()
        .read_line(& mut input)
        .expect("Failed to read line");

    let celsius: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Failed to parse input"),
    };

    let fahrenheit = celsius * 9 / 5 + 32;

    println!();
    println!("{}°C is {}°F", celsius, fahrenheit);
}
