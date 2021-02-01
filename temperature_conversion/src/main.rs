use std::io;

fn main() {
    println!("Input tempelature. (°C)");

    let mut tempelature = String::new();
    io::stdin().read_line(&mut tempelature)
        .expect("Failed to read line");

    let tempelature: f32 = match tempelature.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input.");
            1.0
        },
    };

    println!("Your input value: {} °C", tempelature);
    println!("Degrees Fahrenheit: {} °F", tempelature * 1.8 + 32_f32);
}
