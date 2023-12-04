// was alot worse but then I optimised it a bit w/ help from sm from discord
#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<i32> {
    let mut winning_nums = vec![0; input.lines().count()];
    for (i, line) in input.lines().enumerate() {
        let (want, get) = line.split_once(':').unwrap().1.split_once('|').unwrap();
        let want = want.split_whitespace().collect::<Vec<&str>>();
        for num in get.split_whitespace() {
            if want.contains(&num) {
                winning_nums[i] += 1;
            }
        }
    }
    winning_nums
}

#[aoc(day4, part1)]
pub fn part1(cards: &[i32]) -> i32 {
    cards
        .iter()
        .map(|x| {
            if *x != 0 {
                2_i32.pow((*x - 1) as u32)
            } else {
                0
            }
        })
        .sum()
}

#[aoc(day4, part2)]
pub fn part2(cards: &[i32]) -> usize {
    let mut card_copies: Vec<usize> = vec![1; cards.len()];

    for (i, winnings) in cards.iter().enumerate() {
        for x in (i as i32 + 1)..((i as i32 + 1) + winnings) {
            card_copies[x as usize] += card_copies[i];
        }
    }

    card_copies.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(
            13,
            part1(&input_generator(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            ))
        );
    }
    #[test]
    fn sample2() {
        assert_eq!(
            30,
            part2(&input_generator(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            ))
        );
    }
}
