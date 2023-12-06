/*

THINKING

x-to-y map:
destination_start source_start range_len
...

---

wut that means?

lets say a line is:
50 98 2

when seed is source_start (=98) for the range_len (=2) [98..(98+2)]
98 99
then the corresponding values will be destination_start (=50) for the range_len (=2) [50..(50+2)]
50 51

so [98..(98+2)] -> [50..(50+2)]

or

97  ->  97
98  ->  50
99  ->  51
100 ->  100

keep on doing this for all the sets of numbers under said map

then lets say we get seed 99
that -> soil 51
     -> fert ??
     -> [ ... ]
     -> location x

basically keep on plugging it in into the next map

---
how to do this sensibly?

you cant compute the ranges cus of the massive input/too inefficient

so you need to probably go one by one, plug in the seed, go through the maps:
either all of them
or an algorithm that skips unnessecary
    you just factor in source_start <= seed <= source_start + range_len

    maybe make that a fn (seed, struct) -> bool

lets generate an input of (seeds, maps) (Vec<usize>, Vec<Vec<struct>>)
where struct has destination_start, source_start, range_len as fields

```
then for seed in seeds

    let seed = seed

    for unit conversion in unit conversions
        filter through maps for relevant one(s) (source_start <= seed <= source_start + range_len)
            seed = convert using the map logic (destination_start + (source_start - seed))

            depending on edge cases either continue, or keep going through all valids

    once ur done u should have the answer
```

how does one make this recursive?
idk ill worry about that later

----PART 2----

seeds are ranges

start end start end start end ...

make a func to check where seeds are

there are zeros in the maps that is the key

dest, src, range
0     x    x
we need dest to be 0

if seed == src in that scenareo, we get 0 out

(or it might be more idk, ig ud have to brute force it somehow)

once we get 0 one of the maps says that it starts at 0 meaning it would then be the destination number

if the 0 thing doesnt work, and the range+source for second is shorter than the range on first then we can just do the range number in instead

-

eh nvm i just ended up doing it backwards brute force

j swapped dest and src

-

this one wasnt too bad

*/

use std::cmp::min;

#[derive(Clone, Copy, Debug)]
pub struct Map {
    source_start: usize,
    destination_start: usize,
    range_len: usize,
}

pub fn translate_with_map(seed: usize, m: Map) -> (usize, bool) {
    if !(m.source_start <= seed && seed < m.source_start + m.range_len) {
        return (seed, false);
    }

    (seed - m.source_start + m.destination_start, true)
}

pub fn translate_with_map_backwards(seed: usize, m: Map) -> (usize, bool) {
    if !(m.destination_start <= seed && seed < m.destination_start + m.range_len) {
        return (seed, false);
    }

    (seed - m.destination_start + m.source_start, true)
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> (Vec<usize>, Vec<Vec<Map>>) {
    let mut lines = input.lines();
    (
        lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .trim()
            .split(' ')
            .map(|x| x.parse::<usize>().unwrap())
            .collect(),
        {
            let mut all_maps: Vec<Vec<Map>> = vec![];
            let mut i: usize = 0;
            let mut maps: Vec<Map> = vec![];
            for line in lines {
                if line.chars().nth(0).unwrap_or(' ').is_ascii_digit() {
                    let mut values = line.split_whitespace();
                    maps.push(Map {
                        destination_start: values.next().unwrap().parse().unwrap(),
                        source_start: values.next().unwrap().parse().unwrap(),
                        range_len: values.next().unwrap().parse().unwrap(),
                    });
                }
                if line.trim().is_empty() {
                    if i > 0 {
                        all_maps.push(maps);
                        maps = vec![];
                    }
                    i += 1;
                }
            }
            if !maps.is_empty() {
                all_maps.push(maps);
            }

            // println!("{:?}", all_maps);
            all_maps
        },
    )
}

#[aoc(day5, part1)]
pub fn part1(input: &(Vec<usize>, Vec<Vec<Map>>)) -> usize {
    // println!("{:#?}", input);
    let (seeds, all_maps) = input;

    // println!("{:#?}", all_maps.len());

    let mut min_dist = usize::MAX;

    for seed in seeds {
        let mut unit = *seed;
        let mut changed: bool;
        for maps in all_maps {
            let last_unit = unit;
            // println!("{:#?}", maps);
            for map in maps {
                let x;
                (x, changed) = translate_with_map(last_unit, *map);

                if changed {
                    unit = x;
                    // println!("{}", unit);
                    break;
                }
            }
        }
        min_dist = min(min_dist, unit);
    }

    min_dist
}

pub fn in_seed_range(num: usize, nums: &Vec<usize>) -> bool {
    for i in (0..nums.len()).step_by(2) {
        // dbg!(nums[i]);
        // dbg!(nums[i + 1]);
        // dbg!(num);
        if nums[i] <= num && num < (nums[i] + nums[i + 1]) {
            return true;
        }
    }

    false
}

#[aoc(day5, part2)]
pub fn part2(input: &(Vec<usize>, Vec<Vec<Map>>)) -> usize {
    let (seeds, all_maps) = input;

    let mut location = 0;
    loop {
        let mut unit = location;
        let mut changed: bool;
        for maps in all_maps.iter().rev() {
            let last_unit = unit;
            // println!("{:#?}", maps);
            for map in maps {
                let x;
                (x, changed) = translate_with_map_backwards(last_unit, *map);

                if changed {
                    unit = x;
                    break;
                    // unit = min(unit, x);
                    // println!("{}", unit);
                }
            }
        }
        if in_seed_range(unit, seeds) {
            // dbg!(unit);
            // dbg!(location);
            return location;
        }
        location += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            35,
            part1(&input_generator(
                "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"
            ))
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            46,
            part2(&input_generator(
                "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"
            ))
        );
    }
}
