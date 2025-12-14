use std::time::{SystemTime, UNIX_EPOCH};

mod day01;
mod day02;
fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let input = std::fs::read_to_string("input/day02.txt").unwrap();
    //  println!("Part 1: {}", day01::part1(&input));
    //println!("Part 2: {}", day01::part2(&input));

    // println!("Part 1: {}", day02::part1(&input));
    println!("Part 2: {}", day02::part2(&input));
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Time taken: {} ms", end.as_millis() - start.as_millis());
}
