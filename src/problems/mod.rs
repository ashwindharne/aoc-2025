pub mod day1;
pub mod day2;
pub mod day3;

pub fn run_problem(day: u32) {
    match day {
        1 => day1::main(),
        2 => day2::main(),
        3 => day3::main(),
        _ => println!("Problem {} not implemented yet", day),
    }
}
