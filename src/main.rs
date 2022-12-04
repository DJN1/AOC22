pub mod day1;
pub mod day2;

fn main() {
    println!("Max Value: {:?}", day1::solve_part1());
    println!("Total Max Value: {:?}", day1::solve_part2());
    println!("Score: {:?}", day2::solve_part1());
    println!("Fixed Score: {:?}", day2::solve_part2());
}
