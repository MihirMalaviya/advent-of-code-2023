use std::collections::HashMap;

pub struct Node(String, String);

// #[aoc_generator(day8)]
// pub fn input_generator(input: &str) -> (Vec<usize>, HashMap<String, Node>) {
//     let mut lines = input.lines();
//     let order = lines
//         .next()
//         .unwrap()
//         .trim()
//         .chars()
//         .map(|x| if x == 'L' { 0 } else { 1 })
//         .collect();
//     let mut nodes: HashMap<String, Node> = HashMap::new();
//     lines.skip(1).for_each(|l| {
//         nodes.insert(
//             String::from(&l[0..3]),
//             Node(String::from(&l[7..10]), String::from(&l[12..15])),
//         );
//     });

//     (order, nodes)
// }

#[aoc(day8, part1)]
pub fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let order: Vec<usize> = lines
        .next()
        .unwrap()
        .trim()
        .chars()
        .map(|x| if x == 'L' { 0 } else { 1 })
        .collect();
    let mut nodes: HashMap<String, Node> = HashMap::new();
    lines.skip(1).for_each(|l| {
        nodes.insert(
            String::from(&l[0..3]),
            Node(String::from(&l[7..10]), String::from(&l[12..15])),
        );
    });

    let mut steps = 0;
    let mut thing = String::from("AAA");
    let mut order = order.into_iter().cycle();

    while thing != String::from("ZZZ") {
        thing = match order.next().unwrap() {
            0 => nodes.get(&thing).unwrap().0.clone(),
            1 => nodes.get(&thing).unwrap().1.clone(),
            _ => panic!(),
        };
        steps += 1;
    }

    steps
}

pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

pub fn lcm(nums: Vec<usize>) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let mut nums = nums.clone();
    nums.remove(0);
    let b = lcm(nums);
    (a * b) / gcd(a, b)
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> usize {
    let mut lines = input.lines();
    let order: Vec<usize> = lines
        .next()
        .unwrap()
        .trim()
        .chars()
        .map(|x| if x == 'L' { 0 } else { 1 })
        .collect();
    let mut nodes: HashMap<String, Node> = HashMap::new();
    let mut keys: Vec<String> = Vec::new();
    lines.skip(1).for_each(|l| {
        nodes.insert(
            String::from(&l[0..3]),
            Node(String::from(&l[7..10]), String::from(&l[12..15])),
        );
        keys.push(String::from(&l[0..3]));
    });

    let mut things: Vec<String> = Vec::new();

    keys.iter().for_each(|key| {
        if key.bytes().last().unwrap() == b'A' {
            things.push(key.to_string());
        }
    });

    let mut factors = Vec::new();
    for thing in things {
        let mut steps = 0;
        let mut order = order.iter().cycle();
        let mut thing = thing;
        while thing.bytes().last().unwrap() != b'Z' {
            thing = match order.next().unwrap() {
                0 => nodes.get(&thing).unwrap().0.clone(),
                1 => nodes.get(&thing).unwrap().1.clone(),
                _ => panic!(),
            };
            steps += 1;
        }
        factors.push(steps);
    }

    // println!("{:#?}", factors);
    lcm(factors)

    // while things.iter().any(|key| key.bytes().nth(2).unwrap() != b'Z') {
    //     steps += 1;
    //     let choice = order.next().unwrap();

    //     // println!("{:#?}", things);
    //     // let mut bruh = true;
    //     for thing in &mut things {
    //         // if bruh {
    //         //     bruh = !bruh;
    //         //     println!(
    //         //         "{}\t({}, {})\t{}",
    //         //         thing,
    //         //         nodes.get(thing).unwrap().0,
    //         //         nodes.get(thing).unwrap().1,
    //         //         choice
    //         //     );
    //         // }
    //         // if thing.bytes().last().unwrap() == b'Z' {
    //         //     continue;
    //         // }
    //         *thing = match choice {
    //             0 => nodes.get(thing).unwrap().0.clone(),
    //             1 => nodes.get(thing).unwrap().1.clone(),
    //             _ => panic!(),
    //         };
    //     }
    //     // if steps > 10 {
    //     //     return 0;
    //     // }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1a() {
        assert_eq!(
            2,
            part1(
                "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
            )
        );
    }

    #[test]
    fn example1b() {
        assert_eq!(
            6,
            part1(
                "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
            )
        );
    }

    #[test]
    fn example2a() {
        assert_eq!(
            6,
            part2(
                "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            )
        );
    }
}

/*
use std::collections::HashMap;

pub struct Node(String, String);

// #[aoc_generator(day8)]
// pub fn input_generator(input: &str) -> (Vec<usize>, HashMap<String, Node>) {
//     let mut lines = input.lines();
//     let order = lines
//         .next()
//         .unwrap()
//         .trim()
//         .chars()
//         .map(|x| if x == 'L' { 0 } else { 1 })
//         .collect();
//     let mut nodes: HashMap<String, Node> = HashMap::new();
//     lines.skip(1).for_each(|l| {
//         nodes.insert(
//             String::from(&l[0..3]),
//             Node(String::from(&l[7..10]), String::from(&l[12..15])),
//         );
//     });

//     (order, nodes)
// }

#[aoc(day8, part1)]
pub fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let order: Vec<usize> = lines
        .next()
        .unwrap()
        .trim()
        .chars()
        .map(|x| if x == 'L' { 0 } else { 1 })
        .collect();
    let mut nodes: HashMap<String, Node> = HashMap::new();
    lines.skip(1).for_each(|l| {
        nodes.insert(
            String::from(&l[0..3]),
            Node(String::from(&l[7..10]), String::from(&l[12..15])),
        );
    });

    let mut steps = 0;
    let mut thing = String::from("AAA");
    let mut order = order.into_iter().cycle();

    while thing != String::from("ZZZ") {
        thing = match order.next().unwrap() {
            0 => nodes.get(&thing).unwrap().0.clone(),
            1 => nodes.get(&thing).unwrap().1.clone(),
            _ => panic!(),
        };
        steps += 1;
    }

    steps
}

/* part 2 options

make a struct that replaces thing that stores its index, with 0 index being yet to be assigned
make the first 2 letters number ind

*/
#[aoc(day8, part2)]
pub fn part2(input: &str) -> usize {
    let mut lines = input.lines();
    let order: Vec<usize> = lines
        .next()
        .unwrap()
        .trim()
        .chars()
        .map(|x| if x == 'L' { 0 } else { 1 })
        .collect();
    let mut nodes: HashMap<String, Node> = HashMap::new();
    lines.skip(1).for_each(|l| {
        nodes.insert(
            String::from(&l[0..3]),
            Node(String::from(&l[7..10]), String::from(&l[12..15])),
        );
    });

    let mut steps = 0;
    let mut order = order.into_iter().cycle();

    let mut things: Vec<String> = Vec::new();

    nodes.keys().for_each(|key| {
        if key.bytes().last().unwrap() == b'A' {
            things.push(key.to_string());
        }
    });

    while things.iter().any(|key| key.bytes().nth(2).unwrap() != b'Z') {
        steps += 1;
        let choice = order.next().unwrap();

        // println!("{:#?}", things);
        // let mut bruh = true;
        for thing in &mut things {
            // if bruh {
            //     bruh = !bruh;
            //     println!(
            //         "{}\t({}, {})\t{}",
            //         thing,
            //         nodes.get(thing).unwrap().0,
            //         nodes.get(thing).unwrap().1,
            //         choice
            //     );
            // }
            // if thing.bytes().last().unwrap() == b'Z' {
            //     continue;
            // }
            *thing = match choice {
                0 => nodes.get(thing).unwrap().0.clone(),
                1 => nodes.get(thing).unwrap().1.clone(),
                _ => panic!(),
            };
        }
        // if steps > 10 {
        //     return 0;
        // }
    }
    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1a() {
        assert_eq!(
            2,
            part1(
                "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
            )
        );
    }

    #[test]
    fn example1b() {
        assert_eq!(
            6,
            part1(
                "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
            )
        );
    }

    #[test]
    fn example2a() {
        assert_eq!(
            6,
            part2(
                "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            )
        );
    }
}

*/

/*
use std::collections::HashMap;

pub struct HumanNodeChoices(String, String);
pub struct Choice {
    path: usize,
    content: String,
}
pub struct GhostNodeChoices<'a> {
    l: &'a Choice,
    r: &'a Choice,
}

// #[aoc_generator(day8)]
// pub fn input_generator(input: &str) -> (Vec<usize>, HashMap<String, Node>) {
//     let mut lines = input.lines();
//     let order = lines
//         .next()
//         .unwrap()
//         .trim()
//         .chars()
//         .map(|x| if x == 'L' { 0 } else { 1 })
//         .collect();
//     let mut nodes: HashMap<String, Node> = HashMap::new();
//     lines.skip(1).for_each(|l| {
//         nodes.insert(
//             String::from(&l[0..3]),
//             Node(String::from(&l[7..10]), String::from(&l[12..15])),
//         );
//     });

//     (order, nodes)
// }

#[aoc(day8, part1)]
pub fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let order: Vec<usize> = lines
        .next()
        .unwrap()
        .trim()
        .chars()
        .map(|x| if x == 'L' { 0 } else { 1 })
        .collect();
    let mut nodes: HashMap<String, HumanNodeChoices> = HashMap::new();
    lines.skip(1).for_each(|l| {
        nodes.insert(
            String::from(&l[0..3]),
            HumanNodeChoices(String::from(&l[7..10]), String::from(&l[12..15])),
        );
    });

    let mut steps = 0;
    let mut thing = String::from("AAA");
    let mut order = order.into_iter().cycle();

    while thing != String::from("ZZZ") {
        thing = match order.next().unwrap() {
            0 => nodes.get(&thing).unwrap().0.clone(),
            1 => nodes.get(&thing).unwrap().1.clone(),
            _ => panic!(),
        };
        steps += 1;
    }

    steps
}

/*
part 2
make a struct that replaces thing that stores its index and path, with usize max path being yet to be assigned
*/

#[aoc(day8, part2)]
pub fn part2(input: &str) -> usize {
    let mut lines = input.lines();
    let order: Vec<usize> = lines
        .next()
        .unwrap()
        .trim()
        .chars()
        .map(|x| if x == 'L' { 0 } else { 1 })
        .collect();
    let mut nodes: HashMap<String, GhostNodeChoices> = HashMap::new();
    let mut keylist: Vec<String> = Vec::new();
    lines.skip(1).for_each(|l| {
        nodes.insert(
            String::from(&l[0..3]),
            GhostNodeChoices {
                l: &Choice {
                    content: String::from(&l[7..10]),
                    path: usize::MAX,
                },
                r: &Choice {
                    content: String::from(&l[12..15]),
                    path: usize::MAX,
                },
            },
        );
        keylist.push(String::from(&l[0..3]));
    });

    let mut steps = 0;
    let mut order = order.into_iter().cycle();

    let mut threads: Vec<String> = Vec::new();

    nodes.keys().for_each(|key| {
        if key.bytes().last().unwrap() == b'A' {
            threads.push(key.to_string());
        }
    });

    while threads
        .iter()
        .any(|key| key.bytes().nth(2).unwrap() != b'Z')
    {
        steps += 1;

        // println!("{:#?}", things);
        for thread in &mut threads {
            if thread.bytes().last().unwrap() == b'Z' {
                continue;
            }
            *thread = match order.next().unwrap() {
                0 => nodes.get(thread).unwrap().0.clone(),
                1 => nodes.get(thread).unwrap().1.clone(),
                _ => panic!(),
            };
        }
    }
    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1a() {
        assert_eq!(
            2,
            part1(
                "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
            )
        );
    }

    #[test]
    fn example1b() {
        assert_eq!(
            6,
            part1(
                "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
            )
        );
    }

    #[test]
    fn example2a() {
        assert_eq!(
            6,
            part2(
                "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            )
        );
    }
}

*/
