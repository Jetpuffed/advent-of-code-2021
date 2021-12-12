//! Day 2: Dive!
//!
//! Read problem here: https://adventofcode.com/2021/day/2

use advent_of_code_2021::get_input;

fn main() {
    println!("Part 1 solution: {}", puzzle_1());
    println!("Part 2 solution: {}", puzzle_2());
}

fn puzzle_1() -> i32 {
    let input = get_input::<String>(2).unwrap();

    // (horizontal position, depth)
    let (mut x, mut y) = (0, 0);

    // Iterate through all strings in vector
    for s in input {
        // Trim the trailing and leading whitespace and then split by an ASCII space.
        let s = s.trim().split(' ').collect::<Vec<&str>>();

        // Take the first string slice and assign it to `d` (direction),
        // then take the second string slice and assign it to `n` (integer).
        let (d, n) = (s[0], s[1].parse::<i32>().unwrap());

        // Match the direction to its unique operation for `n`
        match d {
            "forward" => x += n,
            "down" => y += n,
            "up" => y -= n,

            // To satisfy the exhaustive search requirement, we use a wildcard
            // to catch any unrecognized inputs. If this happens, then panic...
            _ => {
                panic!("Unexpected input direction!")
            }
        }
    }

    x * y
}

fn puzzle_2() -> i64 {
    let input = get_input::<String>(2).unwrap();

    // (horizontal position, depth, aim)
    let (mut x, mut y, mut aim) = (0, 0, 0);

    // Iterate through all strings in vector
    for s in input {
        // Trim the trailing and leading whitespace and then split by an ASCII space.
        let s = s.trim().split(' ').collect::<Vec<&str>>();

        // Take the first string slice and assign it to `d` (direction),
        // then take the second string slice and assign it to `n` (integer).
        let (d, n) = (s[0], s[1].parse::<i64>().unwrap());

        // Match the direction to its unique operation for `n`
        match d {
            "forward" => {
                x += n;
                y += aim * n
            }
            "down" => aim += n,
            "up" => aim -= n,

            // To satisfy the exhaustive search requirement, we use a wildcard
            // to catch any unrecognized inputs. If this happens, then panic...
            _ => {
                panic!("Unexpected input direction!")
            }
        }
    }

    x * y
}
