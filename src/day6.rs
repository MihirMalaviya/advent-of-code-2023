// #[aoc_generator(day6)]
// pub fn input_generator(input: &str) -> (Vec<usize>, Vec<usize>) {
//     let mut input = input.trim().lines().map(|x| {
//         x.split_once(':')
//             .unwrap()
//             .1
//             .split_whitespace()
//             .map(|n| n.parse::<usize>().unwrap())
//             .collect()
//     });

//     (input.next().unwrap(), input.next().unwrap())
// }

/*

*/

#[aoc(day6, part1)]
pub fn part1(input: &str) -> u32 {
    // dbg!(input);

    let mut input = input.trim().lines().map(|x| {
        x.split_once(':')
            .unwrap()
            .1
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
    });

    let (times, distances) = (input.next().unwrap(), input.next().unwrap());

    let mut total_possibilities = 1;
    for (time, distance) in times.iter().zip(distances) {
        let mut possibilities = 0;
        for presstime in 1..*time {
            if (presstime as i32 * (*time as i32 - presstime as i32) as i32) > distance as i32 {
                possibilities += 1
            }
        }
        total_possibilities *= possibilities;
    }
    total_possibilities
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> i64 {
    // dbg!(input);

    let input = input.replace(' ', "");

    dbg!(&input);

    let mut input = input
        .trim()
        .lines()
        .map(|x| x.split_once(':').unwrap().1.parse::<i64>().unwrap());

    let (time, distance) = (input.next().unwrap(), input.next().unwrap());

    let mut start_working = 0;
    let mut stop_working = 0;

    let mut presstime = 0;
    while start_working == 0 {
        presstime += 1;
        if (presstime * (time - presstime)) > distance {
            start_working = presstime;
        }
    }
    let mut presstime = time;
    while stop_working == 0 {
        presstime -= 1;
        if (presstime * (time - presstime)) > distance {
            stop_working = presstime;
        }
    }
    // idk y +1 but :shrug:
    stop_working - start_working + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(
            288,
            part1(
                "Time:      7  15   30
Distance:  9  40  200
"
            )
        );
    }
    #[test]
    fn sample2() {
        assert_eq!(
            71503,
            part2(
                "Time:      7  15   30
Distance:  9  40  200
"
            )
        );
    }
}
