fn main() {
    println!("TWELVE DAYS OF CHRISTMAS:");

    for n in 1..13 {
        day_intro(n);
        for m in (1..n+1).rev() {
            gift(
                m,
                if m == 1 && n != 1 {
                    ", and\n"
                } else {
                    "\n"
                }
            )
        }
        println!()
    }
}

fn day_intro(n: u32) {
    let text = match n {
        1 => "First",
        2 => "Second",
        3 => "Third",
        4 => "Fourth",
        5 => "Fifth",
        6 => "Sixth",
        7 => "Seventh",
        8 => "Eighth",
        9 => "Ninth",
        10 => "Tehnth",
        11 => "Eleventh",
        12 => "Twefth",
        _ => ""
    };

    print!("\nOn the {} day of Christmas\nMy true love sent to me", text);
}

fn gift(n: u32, prefix: &str) {
    let gift_text = match n {
        1 => "a partridge in a pear tree",
        2 => "Two turtle doves",
        3 => "Three french hens", 
        4 => "Four calling birds",
        5 => "Five golden rings",
        6 => "Six geese a-laying",
        7 => "Seven swans a-swimming",
        8 => "Eight maids a milking",
        9 => "Nine ladies dancing",
        10 => "Ten lords a-leaping",
        11 => "Eleven pipers piping",
        12 => "Twelve drummers drumming",
        _ => ""
    };
    print!("{}{}", prefix, gift_text);
}
