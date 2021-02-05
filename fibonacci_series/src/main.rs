use std::io;
use std::process;

fn main() {
    println!("Input number.");

    let mut number = String::new();
    io::stdin().read_line(&mut number)
        .expect("Failed to read line");

    let number: i64 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input.");
            process::exit(0);
        },
    };

    ordinary(number);
    recursive_call(number);
}

fn ordinary(number: i64) {
    let mut first_number: i64 = 1;
    let mut second_number: i64 = 1;

    print!("Fibonacci number: ");
    if number == 1 {
        println!("{}", first_number);
        process::exit(0);
    }

    if number == 2 {
        println!("{}", second_number);
        process::exit(0);
    }

    let mut target_number: i64 = 0;
    for _n in 2..number {
        target_number = first_number + second_number;
        first_number = second_number;
        second_number = target_number;
    } 

    println!("{}", target_number);
}

fn recursive_call(number: i64) {
    let ans = recursive(number);
    println!("Fibonacci number: {}", ans);
}

fn recursive(number: i64) -> i64 {
    if number <= 0 {
        return 0;
    }

    if number == 1 || number == 2 {
        return 1;
    }

    recursive(number - 1) + recursive(number - 2)
}
