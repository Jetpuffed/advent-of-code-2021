//! Day 1: Sonar Sweep
//!
//! Read problem here: https://adventofcode.com/2021/day/1

use advent_of_code_2021::get_input;

fn main() {
    println!("Part 1 solution: {}", puzzle_1());
    println!("Part 2 solution: {}", puzzle_2());
}

fn puzzle_1() -> i32 {
    let input = get_input::<i32>(1).unwrap();
    let mut res = 0;

    for i in 1..input.len() {
        res += if input[i - 1] < input[i] { 1 } else { 0 };
    }

    res
}

fn puzzle_2() -> i32 {
    let input = get_input::<i32>(1).unwrap();
    let mut prev = 0;
    let mut res = 0;

    for w in input.windows(3) {
        let sum = w.iter().sum();
        if prev == 0 {
            prev = sum;
            continue;
        }
        res += if prev < sum { 1 } else { 0 };
        prev = sum;
    }

    res
}
