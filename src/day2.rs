pub struct Game {
    red: u32,
    green: u32,
    blue: u32,
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> u32 {
    let mut total = 0;
    for (i, game) in input
        .lines()
        .map(|l| {
            let mut game: Game = Game {
                red: 0,
                green: 0,
                blue: 0,
            };

            for round in l
                .split_once(": ")
                .expect("`: ` split problem")
                .1
                .split("; ")
            {
                for part in round.trim().split(", ") {
                    let mut part = part.split(" ");
                    // &part;
                    let value: u32 = part
                        .next()
                        .expect("empty splitted string")
                        .parse()
                        .expect("cant convert number arg to number");
                    match part.last().expect("empty splitted string") {
                        "red" => game.red = game.red.max(value),
                        "green" => game.green = game.green.max(value),
                        "blue" => game.blue = game.blue.max(value),
                        _ => panic!("problem parsing color name"),
                    }
                }
            }
            game
        })
        .enumerate()
    {
        if game.red <= 12 && game.green <= 13 && game.blue <= 14 {
            total += i + 1;
        }
    }

    total as u32
}

#[aoc(day2, part1, optimized)]
pub fn part1_optimized(input: &str) -> u32 {
    let mut total = 0;
    for (i, game) in input
        .lines()
        .map(|l| {
            let mut game: Game = Game {
                red: 0,
                green: 0,
                blue: 0,
            };

            for part in l
                .split_once(": ")
                .expect("`: ` split problem")
                .1
                .split([';', ','])
            {
                let mut part = part.trim().split(" ");
                // &part;
                let value: u32 = part
                    .next()
                    .expect("empty splitted string")
                    .parse()
                    .expect("cant convert number arg to number");
                match part.last().expect("empty splitted string") {
                    "red" => game.red = game.red.max(value),
                    "green" => game.green = game.green.max(value),
                    "blue" => game.blue = game.blue.max(value),
                    _ => panic!("problem parsing color name"),
                }
            }

            game
        })
        .enumerate()
    {
        if game.red <= 12 && game.green <= 13 && game.blue <= 14 {
            total += i + 1;
        }
    }

    total as u32
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u32 {
    let mut total = 0;
    for game in input.lines().map(|l| {
        let mut game: Game = Game {
            red: 0,
            green: 0,
            blue: 0,
        };

        for round in l
            .split_once(": ")
            .expect("`: ` split problem")
            .1
            .split("; ")
        {
            for part in round.trim().split(", ") {
                let mut part = part.split(" ");
                // &part;
                let value: u32 = part
                    .next()
                    .expect("empty splitted string")
                    .parse()
                    .expect("cant convert number arg to number");
                match part.last().expect("empty splitted string") {
                    "red" => game.red = game.red.max(value),
                    "green" => game.green = game.green.max(value),
                    "blue" => game.blue = game.blue.max(value),
                    _ => panic!("problem parsing color name"),
                }
            }
        }
        game
    }) {
        total += game.red * game.green * game.blue;
    }

    total as u32
}

#[aoc(day2, part2, optimized)]
pub fn part2_optimized(input: &str) -> u32 {
    let mut total = 0;
    for game in input.lines().map(|l| {
        let mut game: Game = Game {
            red: 0,
            green: 0,
            blue: 0,
        };

        for part in l
            .split_once(": ")
            .expect("`: ` split problem")
            .1
            .split([';', ','])
        {
            let mut part = part.trim().split(" ");
            // &part;
            let value: u32 = part
                .next()
                .expect("empty splitted string")
                .parse()
                .expect("cant convert number arg to number");
            match part.last().expect("empty splitted string") {
                "red" => game.red = game.red.max(value),
                "green" => game.green = game.green.max(value),
                "blue" => game.blue = game.blue.max(value),
                _ => panic!("problem parsing color name"),
            }
        }

        game
    }) {
        total += game.red * game.green * game.blue;
    }

    total as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(
            8,
            part1(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            )
        );
    }

    #[test]
    fn sample2() {
        assert_eq!(
            2286,
            part2(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            )
        )
    }
}
