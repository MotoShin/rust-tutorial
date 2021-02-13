// 例題
// 0=無回答、1=男性、2=女性
// 与えられた数値をもとに、性別を文字列で表示する関数
// void  showSeibetsu( int seibetsu )　を作成してください

use std::io;
use std::process;

enum Gender {
    NoAnswer,
    Male,
    Female,
}

fn show_gender(number: u8) {
    match number {
        number if number == Gender::NoAnswer as u8 => {
            println!("This number is \"No Answer\".");
        }
        number if number == Gender::Male as u8 => {
            println!("This number is \"Male\".");
        }
        number if number == Gender::Female as u8 => {
            println!("This number is \"Female\".");
        }
        _ => ()
    }
}

fn main() {
    let mut number = String::new();

    io::stdin().read_line(&mut number)
      .expect("Failed to read line");

    let number: u8 = match number.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("Failed convert u8.");
        process::exit(0);
      },
    };

    show_gender(number);
}
