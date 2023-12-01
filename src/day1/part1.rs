/// 1abc2
/// ^   ^
/// pqr3stu8vwx
///    ^   ^
/// a1b2c3d4e5f
///  ^       ^
/// treb7uchet
///     ^
pub fn run(input: &str) -> anyhow::Result<String> {
    let sum: u32 = input.lines().map(|line| {
        let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
        10 * digits.first().unwrap() + digits.last().unwrap()
    }).sum();
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(run("1122zz").unwrap(), "12");
        assert_eq!(run("zzz1111").unwrap(), "11");
        assert_eq!(run("12ss34").unwrap(), "14");
        assert_eq!(run("asd91sdfs212129asdas").unwrap(), "99");
        assert_eq!(run("a5a").unwrap(), "55");
    }

    #[test]
    fn test_run_multiline() {
        assert_eq!(run("1\n2\n34\n55\n").unwrap(), "122");
        assert_eq!(run(
            "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet"
        ).unwrap(), "142");
    }
}