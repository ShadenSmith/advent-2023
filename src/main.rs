use advent::*;

fn day_01() {
    let part1 = day_01::day_01_a("full-inputs/day_01.txt");
    let part2 = day_01::day_01_b("full-inputs/day_01.txt");
    println!("Day 01");
    println!("  Part 1: {}", part1);
    println!("  Part 2: {} (wrong)", part2);
}

fn day_05() {
    let part1 = day_05::day_05_a("full-inputs/day_05.txt");
    println!("Day 05");
    println!("  Part 1: {}", part1);
}

fn main() {
    // TODO: toggle or make separate examples?
    day_01();
    day_05();
}
