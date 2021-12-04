//! Day 1: Sonar Sweep
//! 
//! Read problem here: https://adventofcode.com/2021/day/1

use advent_of_code_2021::get_input;

fn main()
{
    println!("Part 1 solution: {}", puzzle_1());
}

fn puzzle_1() -> i32
{
    let input = get_input::<i32>(1).unwrap();
    let mut res = 0;

    for i in 1 .. input.len()
    {
        res += if input[i - 1] < input[i] { 1 } else { 0 }
    }

    res
}
