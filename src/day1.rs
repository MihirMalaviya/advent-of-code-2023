#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut digits = line
                .trim()
                .chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| c.to_digit(10).expect("not a digit somehow"));

            let first = digits.next().unwrap_or(0);
            let last = digits.last().unwrap_or(first);
            10 * first + last
        })
        .sum()
}

pub fn get_line_nums_with_words(input: &str) -> Vec<u32> {
    let words = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut nums: Vec<u32> = vec![];
    let mut segment = String::new();
    for c in input.chars() {
        if c.is_ascii_digit() {
            nums.push(c.to_digit(10).expect("not a digit somehow"));
        } else {
            segment.push(c);
            for i in 0..=9 {
                if segment.contains(words[i]) {
                    nums.push(i as u32);
                    segment = segment[segment
                        .find(words[i])
                        .expect("didnt find word that was supposed to be there")
                        + 1..]
                        .to_string();
                    break;
                }
            }
        }
    }
    nums
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let digits = get_line_nums_with_words(line);

            let first = digits.first().expect("no first digit in a line");
            let last = digits.last().expect("no last digit in a line");
            10 * first + last
        })
        .sum()
}

// TODO faster implementation, check "... until reach a valid num and then check ..." until reach a valid num

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            142,
            part1(
                "1abc2
                pqr3stu8vwx
                a1b2c3d4e5f
                treb7uchet"
            )
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            281 + 21,
            part2(
                "two1nine
                eightwothree
                abcone2threexyz
                xtwone3four
                4nineeightseven2
                zoneight234
                7pqrstsixteen
                twone"
            )
        );
    }
}
