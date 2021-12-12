use std::fs;

/// Returns the input for a specified day, where `day`
/// is any 8-bit unsigned integer in the range of `1 ..= 25`.
pub fn get_input<T>(day: u8) -> Result<Vec<T>, std::io::Error>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let path = format!("./data/day-{:02}.txt", day);
    let buf = fs::read_to_string(path)?;
    let mut res = Vec::new();

    for line in buf.lines() {
        res.push(line.parse().unwrap());
    }

    Ok(res)
}
