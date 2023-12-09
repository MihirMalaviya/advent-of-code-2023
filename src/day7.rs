use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Game {
    cards: String,
    bid: usize,
}

impl Game {
    pub fn get_repeating_list(&self) -> Vec<i32> {
        let mut counts: HashMap<u8, i32> = HashMap::new();
        self.cards
            .bytes()
            .for_each(|x| *counts.entry(x).or_insert(0) += 1);
        counts.values().cloned().collect()
    }
    pub fn get_repeating_list_with_wildcards(&self) -> Vec<i32> {
        let mut counts: HashMap<u8, i32> = HashMap::new();
        self.cards
            .bytes()
            .for_each(|x| *counts.entry(x).or_insert(0) += 1);

        if let Some(&amount) = counts.get(&b'1') {
            counts.remove(&b'1');
            let mut v = 0;
            let mut k = &b'0';

            for (key, &value) in counts.iter() {
                if value > v {
                    k = key;
                    v = value;
                } else if value == v {
                    k = k.max(key);
                }
            }

            *counts.entry(*k).or_insert(0) += amount;
        }

        counts.values().cloned().collect()
    }
}

pub fn rank(repeats: Vec<i32>) -> usize {
    // five of a kind
    if repeats.contains(&5) {
        return 7;
    }
    // four of a kind
    if repeats.contains(&4) {
        return 6;
    }
    // full house
    if repeats.contains(&3) && repeats.contains(&2) {
        return 5;
    }
    // three of a kind
    if repeats.contains(&3) && repeats.contains(&1) {
        return 4;
    }
    // two pair
    if repeats.iter().filter(|&&x| x == 2).count() == 2 {
        return 3;
    }
    // one pair
    if repeats.contains(&2) {
        return 2;
    }
    1
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> usize {
    let games = input
        .lines()
        .map(|x| {
            let mut x = x.split_whitespace();
            Game {
                cards: String::from({
                    x.next()
                        .unwrap()
                        .replace('A', "e")
                        .replace('K', "d")
                        .replace('Q', "c")
                        .replace('J', "b")
                        .replace('T', "a")
                }),
                bid: { x.next().unwrap().parse().unwrap() },
            }
        })
        .collect::<Vec<Game>>();

    let mut types: Vec<Vec<Game>> = vec![vec![], vec![], vec![], vec![], vec![], vec![], vec![]];

    for game in games.iter() {
        types[rank(game.get_repeating_list()).checked_sub(1).unwrap()].push(game.clone());
    }
    let games: Vec<Game> = types
        .iter_mut()
        .map(|vec| {
            vec.sort_by(|a, b| (b.cards).cmp(&a.cards));
            Vec::from_iter(vec.clone().into_iter().rev())
        })
        .flatten()
        .collect();

    // println!("{:#?}", types);
    // println!("{:#?}", games);

    let mut total = 0;
    for i in 0..games.len() {
        // println!("{:#?} * {:#?}\t{:#?}", games[i].bid, (i + 1), games[i]);
        total += games[i].bid * (i + 1);
    }
    total
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> usize {
    let games = input
        .lines()
        .map(|x| {
            let mut x = x.split_whitespace();
            Game {
                cards: String::from({
                    x.next()
                        .unwrap()
                        .replace('A', "e")
                        .replace('K', "d")
                        .replace('Q', "c")
                        .replace('J', "1")
                        .replace('T', "a")
                }),
                bid: { x.next().unwrap().parse().unwrap() },
            }
        })
        .collect::<Vec<Game>>();

    // TODO implement this better
    let mut types: Vec<Vec<Game>> = vec![vec![], vec![], vec![], vec![], vec![], vec![], vec![]];

    for game in games.iter() {
        types[rank(game.get_repeating_list_with_wildcards())
            .checked_sub(1)
            .unwrap()]
        .push(game.clone());
    }
    let games: Vec<Game> = types
        .iter_mut()
        .map(|vec| {
            vec.sort_by(|a, b| (b.cards).cmp(&a.cards));
            Vec::from_iter(vec.clone().into_iter().rev())
        })
        .flatten()
        .collect();

    // println!("{:#?}", types);
    // println!("{:#?}", games);

    let mut total = 0;
    for i in 0..games.len() {
        // println!("{:#?} * {:#?}\t{:#?}", games[i].bid, (i + 1), games[i]);
        total += games[i].bid * (i + 1);
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            6440,
            part1(
                "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            )
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            5905,
            part2(
                "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            )
        );
    }
}
