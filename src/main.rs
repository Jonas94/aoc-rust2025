use std::time::{SystemTime, UNIX_EPOCH};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let input = std::fs::read_to_string("input/day05.txt").unwrap();
    //  println!("Part 1: {}", day01::part1(&input));
    //println!("Part 2: {}", day01::part2(&input));

    //  println!("Part 1: {}", day04::part1(&input));
    // println!("Part 2: {}", day04::part2(&input));

    //  println!("Part 2: {}", day03::part2(&input));
    println!("Part 2: {}", day05::part2(&input));
    //326289736908801 too low
    //  347338785050515 Correct
    //349441768253421 too high
    //349441768253398 too high

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Time taken: {} ms", end.as_millis() - start.as_millis());
}
