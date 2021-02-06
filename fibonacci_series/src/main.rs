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

// recursive method call
fn recursive_call(number: i64) {
    let mut v: Vec<i64> = Vec::with_capacity(number as usize);
    for _i in 0..number {
        v.push(-1);
    }
    let ans = recursive(number, &mut v);
    println!("Fibonacci number: {}", ans);
}

// recursive method
fn recursive(number: i64, v: &mut Vec<i64>) -> i64 {
    if number <= 0 {
        return 0;
    }

    if number == 1 || number == 2 {
        return 1;
    }

    if v[(number - 1) as usize] != -1 {
        return v[(number - 1) as usize];
    }

    let ans = recursive(number - 1, v) + recursive(number - 2, v);
    v[(number - 1) as usize] = ans;
    ans
}
