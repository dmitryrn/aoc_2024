// ^ > v <

use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Coord {
    x: usize,
    y: usize,
}


pub fn pt1(input: String) -> usize {
    let mut guard_pos: Option<Coord> = None;
    let mut matrix: Vec<Vec<char>> = input
        .lines()
        .filter(|l| !l.is_empty())
        .enumerate()
        .map(|(y, l)| -> Vec<char> {
            let chars =  l.trim().chars();
            if guard_pos.is_some() {
                return chars.collect()
            }
            let finding = chars.clone().enumerate().find(|(_, c)| "^>v<".contains(*c));
            if finding.is_some() {
                guard_pos = Some(Coord{x: finding.unwrap().0, y});
            }
            chars.collect()
        })
        .collect();

    let mut walked_coords_set = HashSet::new();
    let mut loops  =0 ;

    for y in 0..matrix.len() {
        for x in 0..matrix[0].len() {
            let memo = matrix[y][x];
            if memo == '#' { continue }
            matrix[y][x] = '#';

            let mut guard_pos = guard_pos.clone().unwrap();
            let mut guard_direction = matrix[guard_pos.y][guard_pos.x];
            let mut visited_obstacles = HashSet::new();

            // println!("{} {} {}", matrix_str(&matrix), x, y);
            // println!("{} {}", y, x);

            let mut i = 0;
            loop {
                let obstacle_coord = find_obstacle(
                    &matrix,
                    &mut guard_pos, guard_direction,
                    &mut walked_coords_set,
                    &mut visited_obstacles,
                );
                if obstacle_coord.0.is_some() {
                    // if obstacle_coord.1 == true {
                    //     println!("looped {:?}", obstacle_coord);
                    //     loops += 1;
                    //     break
                    // }
                    turn(&mut guard_direction);
                    visited_obstacles.insert(obstacle_coord.0.clone().unwrap());
                }
                // dbg!(walked_coords.clone());
                // walked_coords_set.extend(walked_coords);

                if obstacle_coord.0.is_none() {
                    break
                }

                i += 1;
                if i > 10000 {
                    // println!("looped {:?}", obstacle_coord);
                    loops += 1;
                    break
                    // panic!("too many iterations")
                }
            }

            matrix[y][x] = memo;
        }
    }

    loops
}

pub fn turn(dir: &mut char) {
    match dir {
        '^' => *dir = '>',
        '>' => *dir = 'v',
        'v' => *dir = '<',
        '<' => *dir = '^',
        _ => panic!("invalid direction"),
    }
}

pub fn find_obstacle(matrix: &Vec<Vec<char>>,
    guard_pos: &mut Coord,
    guard_direction: char,
    walked_coords_set: &mut HashSet<Coord>,
    visited_obstacles: &HashSet<Coord>,
) -> (Option<Coord>, bool) {
    // println!("{} {} {} {}", matrix_str(matrix), guard_pos.x, guard_pos.y, guard_direction);
    // let mut walked_coords = Vec::new();

    let mut loopin = false;
    match guard_direction {
        '^' => {
            let x = guard_pos.x;
            let mut y = guard_pos.y as isize;
            while y >= 0 {
                if matrix[y as usize][x] == '#' {
                    if visited_obstacles.contains(&Coord{x, y: y as usize}) {
                        loopin = true;
                    }
                    return (Some(Coord{x, y: y as usize}), loopin)
                } else {
                    guard_pos.y = y as usize;
                    walked_coords_set.insert(Coord{x, y: y as usize});
                }
                y -= 1;
            }
            (None, loopin)
        }
        'v' => {
            let range: Vec<usize> = (guard_pos.y+1..matrix.len()).collect();
            let x = guard_pos.x;
            for y in range {
                if matrix[y][x] == '#' {
                    if visited_obstacles.contains(&Coord{x, y: y as usize}) {
                        loopin = true;
                    }
                    return (Some(Coord{x, y}), loopin)
                } else {
                    guard_pos.y = y;
                    walked_coords_set.insert(Coord{x, y});
                }
            }
            (None, loopin)
        }
        '<' => {
            let range: Vec<usize> = (0..guard_pos.x).rev().collect();
            let y = guard_pos.y;
            for x in range {
                if matrix[y][x] == '#' {
                    if visited_obstacles.contains(&Coord{x, y: y as usize}) {
                        return (Some(Coord{x: 1000, y: 1000}), loopin)
                    }
                    return (Some(Coord{x, y}), loopin)
                } else {
                    guard_pos.x = x;
                    walked_coords_set.insert(Coord{x, y});
                }
            }
            (None, loopin)
        }
        '>' => {
            let range: Vec<usize> = (guard_pos.x+1..matrix[0].len()).collect();
            let y = guard_pos.y;
            for x in range {
                if matrix[y][x] == '#' {
                    if visited_obstacles.contains(&Coord{x, y: y as usize}) {
                        return (Some(Coord{x: 1000, y: 1000}), loopin)
                    }
                    return (Some(Coord{x, y}), loopin)
                } else {
                    guard_pos.x = x;
                    walked_coords_set.insert(Coord{x, y});
                }
            }
            (None, loopin)
        }
        _ => (None, loopin)
    }
}

pub fn matrix_str(matrix: &Vec<Vec<char>>) -> String {
    let a: String = matrix.iter().fold("".to_string(), |acc, l| {
        let collect: String = l.into_iter().collect();
        acc + "\n" + collect.as_str()
    });
    a
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn pt1_test() {
        let got = pt1("
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
".to_string());
        assert_eq!(got, 6);
    }

    #[test]
#[ignore]
    fn pt1_prod_test() {
        assert_eq!(pt1(fs::read_to_string("./inputs/day6.txt").unwrap()), 1789);
    }
}
