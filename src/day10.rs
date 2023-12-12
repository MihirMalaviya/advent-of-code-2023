use core::panic;
use std::collections::HashSet;

#[derive(Debug)]
struct Pipe {
    coords: (i32, i32),
    pointing_to: ((i32, i32), (i32, i32)),
}

impl Pipe {
    fn from_coords(coords: (i32, i32), tiles: &Vec<Vec<char>>) -> Pipe {
        Pipe {
            coords: coords,
            pointing_to: get_neighbors(coords, &tiles),
        }
    }

    fn from_start_coords(coords: (i32, i32), tiles: &Vec<Vec<char>>) -> Pipe {
        Pipe {
            coords: coords,
            pointing_to: get_starting_neighbors(coords, &tiles),
        }
    }
}

fn get_neighbors(coords: (i32, i32), tiles: &Vec<Vec<char>>) -> ((i32, i32), (i32, i32)) {
    // println!("{}", get_coords(coords, &tiles));
    match get_coords(coords, &tiles) {
        '|' => ((coords.0, coords.1 + 1), (coords.0, coords.1 - 1)),
        '-' => ((coords.0 - 1, coords.1), (coords.0 + 1, coords.1)),
        'L' => ((coords.0, coords.1 - 1), (coords.0 + 1, coords.1)),
        'J' => ((coords.0, coords.1 - 1), (coords.0 - 1, coords.1)),
        '7' => ((coords.0, coords.1 + 1), (coords.0 - 1, coords.1)),
        'F' => ((coords.0, coords.1 + 1), (coords.0 + 1, coords.1)),
        _ => {
            println!("{:#?}\t{}", coords, get_coords(coords, &tiles));
            panic!("not a pipe?")
        }
    }
}

fn get_starting_neighbors(coords: (i32, i32), tiles: &Vec<Vec<char>>) -> ((i32, i32), (i32, i32)) {
    let (x, y) = coords;

    let mut pointing_to: Vec<(i32, i32)> = Vec::new();
    if match get_coords((x, y + 1), tiles) {
        '|' => true,
        'L' => true,
        'J' => true,
        _ => false,
    } {
        pointing_to.push((x, y + 1));
    }
    if match get_coords((x, y - 1), tiles) {
        '|' => true,
        'F' => true,
        '7' => true,
        _ => false,
    } {
        pointing_to.push((x, y - 1));
    }
    if match get_coords((x - 1, y), tiles) {
        '-' => true,
        'F' => true,
        'L' => true,
        _ => false,
    } {
        pointing_to.push((x - 1, y));
    }
    if match get_coords((x + 1, y), tiles) {
        '-' => true,
        'J' => true,
        '7' => true,
        _ => false,
    } {
        pointing_to.push((x + 1, y));
    }

    // println!("{:?}", get_coords((x, y), tiles));
    // println!(
    //     "\n\n
    // {:#?}",
    //     pointing_to
    // );

    assert_eq!(pointing_to.len(), 2);

    (pointing_to[0], pointing_to[1])
}

fn get_coords(coords: (i32, i32), tiles: &Vec<Vec<char>>) -> char {
    tiles[coords.1 as usize][coords.0 as usize]
}

#[aoc_generator(day10)]
fn parse(input: &str) -> (Vec<Vec<char>>, (i32, i32)) {
    let mut tiles = Vec::new();
    let mut starting_position = None;

    for (row_index, line) in input.lines().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        tiles.push(chars);

        if let Some(col_index) = line.chars().position(|c| c == 'S') {
            starting_position = Some((col_index as i32, row_index as i32));
        }
    }

    (tiles, starting_position.unwrap())
}

#[aoc(day10, part1)]
fn part1(input: &(Vec<Vec<char>>, (i32, i32))) -> usize {
    let (tiles, starting_position) = input;

    let pipe = Pipe::from_start_coords(*starting_position, tiles);
    let mut searched: HashSet<(i32, i32)> = HashSet::new();

    // pipe.coords

    // println!("start {}", get_coords(pipe.coords, tiles));
    // println!("start points to {:?}", pipe.pointing_to);
    let mut path: Pipe = Pipe::from_coords(pipe.pointing_to.0, tiles);

    let mut i = 1;

    searched.insert(pipe.coords);
    searched.insert(path.coords);

    loop {
        i += 1;
        // println!("[{i}]");

        path = Pipe::from_coords(
            {
                // println!("{:?}", searched[j]);
                // println!("{:?}", &paths[j].pointing_to);
                match (
                    searched.contains(&path.pointing_to.0),
                    searched.contains(&path.pointing_to.1),
                ) {
                    (true, false) => path.pointing_to.1,
                    (false, true) => path.pointing_to.0,
                    (false, false) => panic!(),
                    (true, true) => break,
                }
            },
            tiles,
        );
        searched.insert(path.coords);
        // println!();

        if get_coords(path.coords, tiles) == 'S' {
            break;
        }
    }
    {
        let mut pipes = vec![vec!['.'; tiles[0].len()]; tiles.len()];
        for (x, y) in searched.iter() {
            pipes[*y as usize][*x as usize] = match tiles[*y as usize][*x as usize] {
                '|' => '│',
                '-' => '─',
                'F' => '┌',
                '7' => '┐',
                'J' => '┘',
                'L' => '└',
                'S' => 'S',
                _ => panic!(),
            }
        }

        println!(
            "{}",
            pipes
                .into_iter()
                .flat_map(|line| line.into_iter().chain(std::iter::once('\n')))
                .collect::<String>()
        );
    }
    i / 2
}

/*

.┌────┐┌┐┌┐┌┐┌─┐....
.│┌──┐││││││││┌┘....
.││.┌┘││││││││└┐....
┌┘└┐└┐└┘└┘││└┘.└─┐..
└──┘.└┐...└┘S┐┌─┐└┐.
....┌─┘..┌┐┌┘│└┐└┐└┐
....└┐.┌┐││└┐│.└┐└┐│
.....│┌┘└┘│┌┘│┌┐│.└┘
....┌┘└─┐.││.││││...
....└───┘.└┘.└┘└┘...

..........
.S──────┐.
.│┌────┐│.
.││....││.
.││....││.
.│└─┐┌─┘│.
.│..││..│.
.└──┘└──┘.
..........

*/

#[aoc(day10, part1, race)]
fn part1_race(input: &(Vec<Vec<char>>, (i32, i32))) -> usize {
    let (tiles, starting_position) = input;

    let pipe = Pipe::from_start_coords(*starting_position, tiles);
    let mut searched: [HashSet<(i32, i32)>; 2] = [HashSet::new(), HashSet::new()];

    // pipe.coords

    // println!("start {}", get_coords(pipe.coords, tiles));
    // println!("start points to {:?}", pipe.pointing_to);
    let mut paths: [Pipe; 2] = [
        Pipe::from_coords(pipe.pointing_to.0, tiles),
        Pipe::from_coords(pipe.pointing_to.1, tiles),
    ];

    let mut i = 0;

    i += 1;

    for j in 0..2 {
        // println!("searched {:?}", searched[j]);
        // println!("paths {:?}", &paths[j].pointing_to);
        searched[j].insert(pipe.coords);
        searched[j].insert(paths[j].coords);
    }

    loop {
        i += 1;
        // println!("[{i}]");

        let mut latest: [(i32, i32); 2] = [(0, 0), (0, 0)];

        for j in 0..2 {
            paths[j] = Pipe::from_coords(
                {
                    // println!("{:?}", searched[j]);
                    // println!("{:?}", &paths[j].pointing_to);
                    match (
                        searched[j].contains(&paths[j].pointing_to.0),
                        searched[j].contains(&paths[j].pointing_to.1),
                    ) {
                        (true, false) => paths[j].pointing_to.1,
                        (false, true) => paths[j].pointing_to.0,
                        (false, false) => panic!(),
                        (true, true) => panic!(),
                    }
                },
                tiles,
            );
            searched[j].insert(paths[j].coords);
            latest[j] = paths[j].coords;
            // println!();
        }

        if latest[0] == latest[1] {
            break;
        }
    }
    i
}

fn shoelace(points: Vec<(f32, f32)>) -> i32 {
    let mut area: f32 = 0.0;
    for x in 0..points.len() {
        area += points[x].0 * points[(x + 1) % (points.len())].1;
        area -= points[x].1 * points[(x + 1) % (points.len())].0;
    }
    (area.abs() / 2.0) as i32
}

// TODO part 2 is unfinished; i manually counted them with paint.net
#[aoc(day10, part2)]
fn part2(input: &(Vec<Vec<char>>, (i32, i32))) -> i32 {
    let (tiles, starting_position) = input;

    let pipe = Pipe::from_start_coords(*starting_position, tiles);
    let mut searched: HashSet<(i32, i32)> = HashSet::new();

    // pipe.coords

    // println!("start {}", get_coords(pipe.coords, tiles));
    // println!("start points to {:?}", pipe.pointing_to);
    let mut path: Pipe = Pipe::from_coords(pipe.pointing_to.0, tiles);

    let mut points: Vec<(f32, f32)> = Vec::new();

    searched.insert(pipe.coords);

    // if pipe.pointing_to.0 .0 != pipe.pointing_to.1 .0
    //     && pipe.pointing_to.0 .1 != pipe.pointing_to.1 .1
    // {
    //     points.push(pipe.coords);
    //     // panic!();
    // }

    searched.insert(path.coords);

    loop {
        // println!("[{i}]");

        if let Some(coords) = match get_coords(path.coords, tiles) {
            'L' => Some((path.coords.0 as f32 + 0.5, path.coords.1 as f32 - 0.5)),
            'J' => Some((path.coords.0 as f32 - 0.5, path.coords.1 as f32 - 0.5)),
            '7' => Some((path.coords.0 as f32 - 0.5, path.coords.1 as f32 + 0.5)),
            'F' => Some((path.coords.0 as f32 + 0.5, path.coords.1 as f32 + 0.5)),
            _ => None,
        } {
            points.push(coords);
        }

        path = Pipe::from_coords(
            {
                // println!("{:?}", searched[j]);
                // println!("{:?}", &paths[j].pointing_to);
                match (
                    searched.contains(&path.pointing_to.0),
                    searched.contains(&path.pointing_to.1),
                ) {
                    (true, false) => path.pointing_to.1,
                    (false, true) => path.pointing_to.0,
                    (false, false) => panic!(),
                    (true, true) => break,
                }
            },
            tiles,
        );
        searched.insert(path.coords);
        // println!();

        if get_coords(path.coords, tiles) == 'S' {
            break;
        }
    }

    {
        let mut pipes = vec![vec!['.'; tiles[0].len()]; tiles.len()];
        for (x, y) in searched.iter() {
            pipes[*y as usize][*x as usize] = match tiles[*y as usize][*x as usize] {
                '|' => '│',
                '-' => '─',
                'F' => '┌',
                '7' => '┐',
                'J' => '┘',
                'L' => '└',
                'S' => 'S',
                _ => panic!(),
            }
        }

        for (x, y) in points.iter() {
            pipes[*y as usize][*x as usize] = 'X';
        }

        println!(
            "{}",
            pipes
                .into_iter()
                .flat_map(|line| line.into_iter().chain(std::iter::once('\n')))
                .collect::<String>()
        );
    }

    // println!("{:#?}", points);
    shoelace(points)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1a() {
        assert_eq!(
            4,
            part1(&parse(
                "-L|F7
7S-7|
L|7||
-L-J|
L|-JF"
            ))
        );
    }

    #[test]
    fn example1b() {
        assert_eq!(
            8,
            part1(&parse(
                ".7-F7-
..FJ|7
.SJLL7
.|F--J
.LJ.LJ"
            ))
        );
    }

    //     #[test]
    //     fn example2a() {
    //         assert_eq!(
    //             4,
    //             part2(&parse(
    //                 "..........
    // .S------7.
    // .|F----7|.
    // .||OOOO||.
    // .||OOOO||.
    // .|L-7F-J|.
    // .|II||II|.
    // .L--JL--J.
    // .........."
    //             ))
    //         );
    //     }
    //     #[test]
    //     fn example2b() {
    //         assert_eq!(
    //             10,
    //             part2(&parse(
    //                 "......................
    // .FF7FSF7F7F7F7F7F---7.
    // .L|LJ||||||||||||F--J.
    // .FL-7LJLJ||||||LJL-77.
    // .F--JF--7||LJLJ7F7FJ-.
    // .L---JF-JLJ.||-FJLJJ7.
    // .|F|F-JF---7F7-L7L|7|.
    // .|FFJF7L7F-JF7|JL---7.
    // .7-L-JL7||F7|L7F-7F7|.
    // .L.L7LFJ|||||FJL7||LJ.
    // .L7JLJL-JLJLJL--JLJ.L.
    // ......................"
    //             ))
    //         );
    //     }

    //     #[test]
    //     fn example2c() {
    //         assert_eq!(
    //             1,
    //             part2(&parse(
    //                 "-L|F7
    // 7S-7|
    // L|7||
    // -L-J|
    // L|-JF"
    //             ))
    //         );
    //     }
}
