use std::fs;

/// Returns the input for a specified day, where `day`
/// is any 8-bit unsigned integer in the range of `1 ..= 25`.
pub fn get_input(day: u8) -> Result<String, std::io::Error>
{
    let path = format!("./data/day-{:02}.txt", day);

    fs::read_to_string(path)
}
