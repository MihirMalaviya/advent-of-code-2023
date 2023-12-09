/*
PLAN
find next symbol in loop (every non period/digit)

look at every pos around the symbol, and if its a digit, look next to the digit on both sides until exhausted
     make sure that u dont repeat digits somehow
         maybe just do and then check where the digit was and if same place ignore

     so i will need:
         a get full digit from coordinate function
             returns digit starting index
             returns digit as an int

then just add them up
*/

#[aoc_generator(day3)]
pub fn parse(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|x| x.trim().bytes().collect()).collect()
}

pub fn is_symbol(byte: u8) -> bool {
    byte != b'.' && !byte.is_ascii_digit()
}

pub fn get_num_index(line: &Vec<u8>, index: (usize, usize)) -> (usize, usize) {
    let mut x: i32 = index.0 as i32;
    while line[x as usize].is_ascii_digit() {
        if x != 0 {
            x -= 1;
        } else {
            if x == 0 {
                return (0, index.1);
            }
        }
    }
    (x as usize + 1, index.1)
}

pub fn get_full_num(line: &Vec<u8>, index: usize) -> usize {
    let mut index = index;
    let mut digits: Vec<u8> = vec![];
    while line[index].is_ascii_digit() {
        digits.push(line[index]);
        if index != line.len() - 1 {
            index += 1;
        } else {
            break;
        }
    }
    String::from_utf8(digits)
        .expect("digit string wasn't parsable")
        .parse::<usize>()
        .expect("digit string wasn't parsable")
}

#[aoc(day3, part1)]
pub fn part1(input: &[Vec<u8>]) -> usize {
    let mut sum = 0;
    let mut exhausted_indexes: Vec<(usize, usize)> = vec![];

    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if !is_symbol(*c) {
                continue;
            }

            for shift_y in 0..=2 {
                for shift_x in 0..=2 {
                    // if on symbol, or out of bounds
                    if (shift_x == 1 && shift_y == 1)
                        || y + 1 < shift_y
                        || x + 1 < shift_x
                        || y + shift_y > input.len()
                        || x + shift_x > line.len()
                    {
                        continue;
                    }
                    let index = (
                        x.checked_add_signed(shift_x as isize - 1)
                            .expect("index x add has problem"),
                        y.checked_add_signed(shift_y as isize - 1)
                            .expect("index y add has problem"),
                    );

                    let neighbor = input[index.1][index.0];
                    if neighbor.is_ascii_digit() {
                        let num_index = get_num_index(&input[index.1], index);
                        if exhausted_indexes.contains(&num_index) {
                            continue;
                        } else {
                            sum += get_full_num(&input[num_index.1], num_index.0);
                            exhausted_indexes.push(num_index);
                        }
                    }
                }
            }
        }
    }
    sum
}

#[aoc(day3, part2)]
pub fn part2(input: &[Vec<u8>]) -> usize {
    let mut sum = 0;
    let mut exhausted_indexes: Vec<(usize, usize)> = vec![];

    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c != b'*' {
                continue;
            }
            let mut nums: Vec<usize> = vec![];

            // let mut adjacent_nums = 0;
            for shift_y in 0..=2 {
                for shift_x in 0..=2 {
                    // if on symbol, or out of bounds
                    if (shift_x == 1 && shift_y == 1)
                        || y + 1 < shift_y
                        || x + 1 < shift_x
                        || y + shift_y > input.len()
                        || x + shift_x > line.len()
                    {
                        continue;
                    }
                    let index = (
                        x.checked_add_signed(shift_x as isize - 1)
                            .expect("index x add has problem"),
                        y.checked_add_signed(shift_y as isize - 1)
                            .expect("index y add has problem"),
                    );

                    let neighbor = input[index.1][index.0];
                    if neighbor.is_ascii_digit() {
                        let num_index = get_num_index(&input[index.1], index);
                        if !exhausted_indexes.contains(&num_index) {
                            nums.push(get_full_num(&input[num_index.1], num_index.0));
                            exhausted_indexes.push(num_index);
                        }
                    }
                }
            }
            if nums.len() == 2 {
                // println!("[{}]\t{} * {} = {}", c, nums[0], nums[1], nums[0] * nums[1]);
                sum += nums[0] * nums[1];
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            4361,
            part1(&parse(
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            ))
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            467835,
            part2(&parse(
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            ))
        );
    }
}
