//! Day 3: Binary Diagnostic
//! 
//! Read problem here: https://adventofcode.com/2021/day/3

use advent_of_code_2021::get_input;

fn main()
{
    println!("{}", puzzle_1());
}

fn puzzle_1() -> u32
{
    let input = get_input::<String>(3).unwrap().iter().map(|x| u16::from_str_radix(x, 2).unwrap()).collect::<Vec<_>>();

    // Initialize result value.
    let mut res = 0;

    // Because we already know the total bits in each value,
    // we can just iterate through the range of bits.
    for i in 0 .. 12
    {
        // Shift 1 left by the current index to get the bit we want to compare.
        let bit = 1 << i;

        // Initialize temporary values to hold 0 and 1 counts.
        let (mut zeros, mut ones) = (0, 0);

        // Iterate through the values in input.
        for n in &input
        {
            // AND `bit` with `n` to determine if bit is set or not.
            if (bit & n) != 0 { ones += 1 } else { zeros += 1 }
        }

        // If 1 is the most common bit, XOR assign the bit to that position
        if zeros < ones { res ^= bit }
    }

    // Bitwise NOT the return value to get the epsilon rate, then XOR
    // bits 12 - 15 with a mask to clear the bits that were set.
    (res as u32) * (!res ^ 0xF000) as u32
}
