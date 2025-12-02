pub mod day1;

pub fn run_problem(day: u32) {
    match day {
        1 => day1::main(),
        _ => println!("Problem {} not implemented yet", day),
    }
}
