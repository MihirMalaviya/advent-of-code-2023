#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn expand(values: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut all = vec![values.clone()];

    while all.last().unwrap().iter().any(|v| *v != 0) {
        let mut next_down = Vec::new();
        // TODO make more idiomatic with chaining with .windows(2).map( ... )
        for i in 1..all.last().unwrap().len() {
            next_down.push(all.last().unwrap()[i] - all.last().unwrap()[i - 1])
        }
        all.push(next_down.clone());
    }
    all
}

fn extrapolate_forward(values: &Vec<i32>) -> i32 {
    let all = expand(values);

    let mut last = 0;
    for i in (0..(all.len() - 1)).rev() {
        last = all[i].last().unwrap() + last;
    }

    last
}

fn extrapolate_backward(values: &Vec<i32>) -> i32 {
    let all = expand(values);

    let mut last = 0;
    for i in (0..(all.len() - 1)).rev() {
        last = all[i][0] - last;
    }

    last
}

#[aoc(day9, part1)]
fn part1(input: &Vec<Vec<i32>>) -> i32 {
    input
        .iter()
        .map(|history| extrapolate_forward(history))
        .sum()
}

#[aoc(day9, part2)]
fn part2(input: &Vec<Vec<i32>>) -> i32 {
    input
        .iter()
        .map(|history| extrapolate_backward(history))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            114,
            part1(&parse(
                "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            ))
        );
    }
    #[test]
    fn example2() {
        assert_eq!(
            2,
            part2(&parse(
                "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            ))
        );
    }
}
