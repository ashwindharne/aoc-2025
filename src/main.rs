mod problems;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run <day_number>");
        println!("Example: cargo run 1");
        return;
    }

    let day: u32 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: Please provide a valid day number");
            return;
        }
    };

    problems::run_problem(day);
}
