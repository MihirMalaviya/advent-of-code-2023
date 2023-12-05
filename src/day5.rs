/*
x-to-y map:
destination_start source_start range_len
...

---

wut that means?

lets say a line is:
50 98 2

when seed is source_start (=98) for the range_len (=2) [98..(98+2)]
then the corresponding values will be destination_start (=50) for the range_len (=2) [50..(50+2)]

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

*/

// #[aoc(day1, part1)]
// pub fn part1(input: &str) -> u32 {}

// #[aoc(day1, part2)]
// pub fn part2(input: &str) -> u32 {}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     // #[test]
//     // fn sample1() {
//     //     assert_eq!(0, part1(""));
//     // }
// }
