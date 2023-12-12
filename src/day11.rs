use std::{
    cmp::{max, min},
    collections::HashSet,
};

#[aoc_generator(day11)]
fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect()
}

// fn swap_rows_and_cols(matrix: &mut Vec<Vec<char>>) {
//     let num_rows = matrix.len();
//     let num_cols = matrix[0].len();

//     let mut swapped_matrix = vec![vec![' '; num_rows]; num_cols];

//     for i in 0..num_rows {
//         for j in 0..num_cols {
//             swapped_matrix[j][i] = matrix[i][j];
//         }
//     }

//     *matrix = swapped_matrix;
// }

fn find_galaxy_distances_after_expansion(universe: &[Vec<char>], expansion: i64) -> i64 {
    let mut rows_with_galaxies: HashSet<i64> = HashSet::new();
    let mut cols_with_galaxies: HashSet<i64> = HashSet::new();

    for row in 0..universe.len() {
        for col in 0..universe[0].len() {
            if universe[row][col] == '#' {
                rows_with_galaxies.insert(row as i64);
                cols_with_galaxies.insert(col as i64);
            }
        }
    }

    // let mut offset = 0;
    // for row in 0..universe.len() {
    //     if !rows_with_galaxies.contains(&row) {
    //         universe.insert(row + offset, vec!['.'; universe[0].len()]);
    //         offset += 1;
    //     }
    // }

    // swap_rows_and_cols(&mut universe);

    // let mut offset = 0;
    // for col in 0..universe.len() {
    //     if !cols_with_galaxies.contains(&col) {
    //         universe.insert(col + offset, vec!['.'; universe[0].len()]);
    //         offset += 1;
    //     }
    // }

    // swap_rows_and_cols(&mut universe);

    // let rows_without_galaxies: HashSet<usize> = (0..universe.len())
    //     .collect::<HashSet<usize>>()
    //     .difference(&rows_with_galaxies)
    //     .cloned()
    //     .collect();
    // let cols_without_galaxies: HashSet<usize> = (0..universe[0].len())
    //     .collect::<HashSet<usize>>()
    //     .difference(&cols_with_galaxies)
    //     .cloned()
    //     .collect();

    // println!(
    //     "{}",
    //     universe
    //         .iter()
    //         .map(|row| row.iter().collect::<String>())
    //         .collect::<Vec<String>>()
    //         .join("\n")
    // );

    let mut galaxies: Vec<(i64, i64)> = Vec::new();

    for row in 0..universe.len() {
        for col in 0..universe[0].len() {
            if universe[row][col] == '#' {
                galaxies.push((row as i64, col as i64))
            }
        }
    }

    let mut total_distance = 0;
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            for row in min(galaxies[j].0, galaxies[i].0)..max(galaxies[j].0, galaxies[i].0) {
                if !rows_with_galaxies.contains(&row) {
                    total_distance += expansion
                }
            }

            for col in min(galaxies[j].1, galaxies[i].1)..max(galaxies[j].1, galaxies[i].1) {
                if !cols_with_galaxies.contains(&col) {
                    total_distance += expansion
                }
            }

            total_distance += (galaxies[j].0 - galaxies[i].0).abs();
            total_distance += (galaxies[j].1 - galaxies[i].1).abs();
        }
    }

    total_distance
}

#[aoc(day11, part1)]
fn part1(universe: &[Vec<char>]) -> i64 {
    find_galaxy_distances_after_expansion(universe, 1)
}

#[aoc(day11, part2)]
fn part2(universe: &[Vec<char>]) -> i64 {
    find_galaxy_distances_after_expansion(universe, 1_000_000 - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            374,
            part1(&parse(
                "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
            ))
        );
    }
    #[test]
    fn example2() {
        assert_eq!(
            8410,
            find_galaxy_distances_after_expansion(
                &parse(
                    "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
                ),
                100 - 1
            )
        );
    }
}
