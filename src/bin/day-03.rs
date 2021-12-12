//! Day 3: Binary Diagnostic
//!
//! Read problem here: https://adventofcode.com/2021/day/3

use advent_of_code_2021::get_input;

fn main() {
    println!("{}", puzzle_1());
    println!("{:?}", puzzle_2());
}

fn puzzle_1() -> i32 {
    let input = get_input::<String>(3).unwrap();
    let mut gam_buf = String::new(); // Buffer for building gamma rate.
    let mut eps_buf = String::new(); // Buffer for building epsilon rate.

    for col in 0..12 {
        let mut tmp_0 = 0; // # of zeros in column.
        let mut tmp_1 = 0; // # of ones in column.

        for row in &input {
            if row.as_bytes()[col] == b'0' {
                tmp_0 += 1
            } else {
                tmp_1 += 1
            }
        }

        if tmp_0 < tmp_1 {
            gam_buf.push('1'); // Most common bit in column.
            eps_buf.push('0'); // Least common bit in column.
        } else {
            gam_buf.push('0'); // Most common bit in column.
            eps_buf.push('1'); // Least common bit in column.
        }
    }

    i32::from_str_radix(&gam_buf, 2).unwrap() * i32::from_str_radix(&eps_buf, 2).unwrap()
}

fn puzzle_2() -> i32 {
    let input = get_input::<String>(3).unwrap();
    let oxy = _puzzle_2(input.clone(), true);
    let co2 = _puzzle_2(input.clone(), false);

    oxy * co2
}

fn _puzzle_2(mut input: Vec<String>, mode: bool) -> i32 {
    let cmp_byte = if mode { [b'1', b'0'] } else { [b'0', b'1'] };

    for col in 0..12 {
        if input.len() <= 2 {
            println!("{:?}", input);
            return i32::from_str_radix(&input[1], 2).unwrap()
        }

        let mut tmp_0 = 0;
        let mut tmp_1 = 0;

        for row in &input {
            if row.as_bytes()[col] == b'0' {
                tmp_0 += 1
            } else {
                tmp_1 += 1
            }
        }

        if tmp_0 <= tmp_1 {
            input = input
                .iter()
                .filter_map(|x| {
                    if x.as_bytes()[col] == cmp_byte[0] {
                        Some(x.to_string())
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            input = input
                .iter()
                .filter_map(|x| {
                    if x.as_bytes()[col] == cmp_byte[1] {
                        Some(x.to_string())
                    } else {
                        None
                    }
                })
                .collect()
        }
    }

    panic!()
}
