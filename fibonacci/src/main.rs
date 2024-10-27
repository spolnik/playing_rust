use std::io;

fn main() {
    println!("Fibonacci sequence generator");
    println!("Enter the number of terms you want to generate: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Not a number: {}", input.trim()),
    };

    // calculate the fibonacci sequence
    let result: u128 = match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a = 0;
            let mut b = 1;
            let mut c = 0;
            for _ in 2..=n {
                c = a + b;
                a = b;
                b = c;
            }
            c
        }
    };

    println!("The {}th term of the Fibonacci sequence is: {}", n, result);
}
