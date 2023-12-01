/// replaces words with digits and uses part1::run to calculate the sum
pub fn run(input: &str) -> anyhow::Result<String> {
    let numbers = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let digits = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let lookup: Vec<_> = numbers.iter().enumerate().chain(digits.iter().enumerate()).collect();
    let sum: usize = input.lines().filter_map(|line| {
        let first = lookup.iter()
            .filter_map(|(i, &word)|
                line.find(word).map(|pos| (pos, i))
            ).min_by_key(|(pos, _)| *pos).map(|(_, &i)| i)?;
        let last = lookup.iter()
            .filter_map(|(i, &word)|
                line.rfind(word).map(|pos| (pos, i))
            ).max_by_key(|(pos, _)| *pos).map(|(_, &i)| i)?;
        Some(first * 10 + last)
    }).sum();

    Ok(sum.to_string())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(run("\n").unwrap(), "0");
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

    #[test]
    fn test_with_words_as_digits() {
        assert_eq!(run("one1two2three3").unwrap(), "13");
        assert_eq!(run("one1two2three3\nfour4five5six6").unwrap(), "59");
        assert_eq!(run("one\none").unwrap(), "22");
        // WTF is this correct?
        assert_eq!(run("twone").unwrap(), "21");
        // And this isn't?
        // assert_eq!(run("twone").unwrap(), "22");

        assert_eq!(run("twoneight\none").unwrap(), "39");
        assert_eq!(run("twoneight9\none").unwrap(), "40");
        assert_eq!(run("
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen"
        ).unwrap(), "281");
    }

    #[test]
    fn test_with_words_as_digits_2() {
        assert_eq!(run("
            fivepqxlpninevh2xxsnsgg63pbvdnqptmg
            eight8zlctbmsixhrvbpjb84nnmlcqkzrsix
            hkxqfrqmsixfplbkpkdfzzszjxmdjtdkjlprrvr3gghlrqckqtbng
            zkjkctxvssix1dqb22five"
        ).unwrap(), "267");
    }


    #[test]
    fn convert_all_digits() {
        assert_eq!(run("\nzero\n").unwrap(), "0");
        assert_eq!(run("\none\n").unwrap(), "11");
        assert_eq!(run("\ntwo\n").unwrap(), "22");
        assert_eq!(run("\nthree\n").unwrap(), "33");
        assert_eq!(run("\nfour\n").unwrap(), "44");
        assert_eq!(run("\nfive\n").unwrap(), "55");
        assert_eq!(run("\nsix\n").unwrap(), "66");
        assert_eq!(run("\nseven\n").unwrap(), "77");
        assert_eq!(run("\neight\n").unwrap(), "88");
        assert_eq!(run("\nnine\n").unwrap(), "99");
        assert_eq!(run("\n0\n").unwrap(), "0");
        assert_eq!(run("\n1\n").unwrap(), "11");
        assert_eq!(run("\n2\n").unwrap(), "22");
        assert_eq!(run("\n3\n").unwrap(), "33");
        assert_eq!(run("\n4\n").unwrap(), "44");
        assert_eq!(run("\n5\n").unwrap(), "55");
        assert_eq!(run("\n6\n").unwrap(), "66");
        assert_eq!(run("\n7\n").unwrap(), "77");
        assert_eq!(run("\n8\n").unwrap(), "88");
        assert_eq!(run("\n9\n").unwrap(), "99");
    }
}