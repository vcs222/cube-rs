use std::env;
use rand::Rng;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut len: u8 = if args.len() == 2 {
        args[1].parse::<u8>().unwrap()
    } else {
        25
    };

    let directions = ["R", "R'", "L", "L'", "U", "U'", "D", "D'", "F", "F'", "B", "B'"];
    let mut sequence: String = String::from("");
    let mut next = 0;
    let mut from = 0;
    let mut to = 12;

    while len > 0 {
        next = (next + rand::thread_rng().gen_range(from, to)) % 12;
        sequence += directions[next];
        
        if next % 2 == 1 {
            from = 1;
            to = 11;
        } else {
            from = 2;
            to = 12;
            if rand::thread_rng().gen_range(1, 3) == 2 {
                sequence += "2";
            }
        }
        sequence += " ";

        len -= 1;
    }

    println!("{}", sequence);
}
