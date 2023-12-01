use nom::character::is_digit;

/// 1abc2
/// ^   ^
/// pqr3stu8vwx
///    ^   ^
/// a1b2c3d4e5f
///  ^       ^
/// treb7uchet
///     ^
pub fn run(input: &str) -> anyhow::Result<String> {
    let mut sum: u64 = 0;
    for line in input.lines() {
        let mut is_first = true;
        let mut current_digit: Option<u8> = None;
        line.bytes().filter(|c| is_digit(*c)).for_each(|c| {
            let d: u8 = c - b'0';
            current_digit = Some(d);
            if is_first {
                sum += d as u64;
                is_first = false;
            }
        });
        match current_digit {
            None => unreachable!(),
            Some(d) => sum += d as u64,
        }
    }
    Ok(sum.to_string())
}
