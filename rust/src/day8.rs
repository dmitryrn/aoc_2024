use std::collections::{HashMap, HashSet};

type Antennas = HashMap<char, Vec<(usize, usize)>>;
type Matrix = Vec<Vec<char>>;

pub fn parse(input: &str) -> (Matrix, Antennas) {
    let mut antennas: Antennas = HashMap::new();

    let m = input
        .lines()
        .filter(|l| !l.is_empty())
        .enumerate()
        .map(|(y, l)| {
            let chars: Vec<char> = l.trim().chars().collect();
            for x in 0..chars.len() {
                let c = chars[x];
                if c != '.' {
                    let coords = antennas.get_mut(&c);
                    if coords.is_some() {
                        coords.unwrap().push((x, y));
                    } else {
                        antennas.insert(c, vec![(x, y)]);
                    }
                }
            }
            chars
        })
        .collect();
    (m, antennas)
}

pub fn matrix_str(matrix: &Vec<Vec<char>>) -> String {
    let a: String = matrix.iter().fold("".to_string(), |acc, l| {
        let collect: String = l.into_iter().collect();
        acc + "\n" + collect.as_str()
    });
    a
}

pub fn pt_1(matrix: &Matrix, antennas: &Antennas) -> usize {
    let mut matrix_debug = matrix.clone();
    println!("{} {}", matrix_debug.len(), matrix_debug[0].len());
    let mut antinode_locations_set: HashSet<(usize, usize)> = HashSet::new();
    for c in antennas.keys() {
        let antennas = antennas.get(c).unwrap();
        for i in 0..antennas.len() {
            for j in 0..antennas.len() {
                if i != j {
                    let a1 = antennas[i];
                    let a2 = antennas[j];
                    let antinode1 = cast(tuple_u_to_i(a1), tuple_u_to_i(a2));
                    let antinode2 = cast(tuple_u_to_i(a2), tuple_u_to_i(a1));
                    let ant1_in_bounds = in_bounds(&matrix, antinode1);
                    let ant2_in_bounds = in_bounds(&matrix, antinode2);
                    println!("{c}  {a1:?} {a2:?}  {antinode1:?}, {antinode2:?} {ant1_in_bounds} {ant2_in_bounds}");
                    if ant1_in_bounds {
                        matrix_debug[antinode1.1 as usize][antinode1.0 as usize] = '#';
                        antinode_locations_set.insert(tuple_i_to_u(antinode1));
                    }
                    if ant2_in_bounds {
                        matrix_debug[antinode2.1 as usize][antinode2.0 as usize] = '#';
                        antinode_locations_set.insert(tuple_i_to_u(antinode2));
                    }
                }
            }
        }
    }

    println!("{}", matrix_str(&matrix_debug));

    antinode_locations_set.into_iter().count()
}

pub fn pt_2(matrix: &Matrix, antennas: &Antennas) -> usize {
    let mut matrix_debug = matrix.clone();
    // for (c, antennas) in antennas {
    //     println!("{c} {:?}", antennas.len());
    // }
    let mut antinode_locations_set: HashSet<(usize, usize)> = HashSet::new();
    for c in antennas.keys() {
        let antennas = antennas.get(c).unwrap();
        for i in 0..antennas.len() {
            for j in 0..antennas.len() {
                if i != j {
                    let a1 = antennas[i];
                    let a2 = antennas[j];
                    antinode_locations_set.insert(a1);
                    antinode_locations_set.insert(a2);
                    let mut result1 = Vec::new();
                    let mut result2 = Vec::new();
                    cast_in_bounds(matrix, &mut result1, tuple_u_to_i(a1), tuple_u_to_i(a2));
                    cast_in_bounds(matrix, &mut result2, tuple_u_to_i(a2), tuple_u_to_i(a1));
                    println!("{c}  {a1:?} {a2:?}  {result1:?} {result2:?}");

                    for antinode in [result1, result2].concat() {
                        let antinode = tuple_i_to_u(antinode);
                        antinode_locations_set.insert(antinode);
                    }
                }
            }
        }
    }

    for antinode in &antinode_locations_set {
        matrix_debug[antinode.1][antinode.0] = '#';
    }

    println!("{}", matrix_str(&matrix_debug));

    antinode_locations_set.into_iter().count()
}

pub fn cast_in_bounds(m: &Matrix, result: &mut Vec<(isize, isize)>, a: (isize, isize), b: (isize, isize)) {
    let next = cast(a, b);
    if !in_bounds(m, next) {
        return;
    }

    result.push(next);

    cast_in_bounds(m, result, b, next);
}

pub fn in_bounds(m: &Matrix, coord: (isize, isize)) -> bool {
    let (x, y) = coord;

    y < m.len() as isize && y >= 0 
        && x < m[0].len() as isize && x >= 0 
}

pub fn tuple_u_to_i(t: (usize, usize)) -> (isize, isize) {
    (t.0 as isize, t.1 as isize)
}

pub fn tuple_i_to_u(t: (isize, isize)) -> (usize, usize) {
    assert!(t.0 >= 0);
    assert!(t.1 >= 0);
    (t.0 as usize, t.1 as usize)
}

pub fn cast(a: (isize, isize), b: (isize, isize)) -> (isize, isize) {
    let (x1, y1) = a;
    let (x2, y2) = b;

    let diff_x = x2 - x1;
    let new_x = x2 + diff_x;

    let diff_y = y2 - y1;
    let new_y = y2 + diff_y;

    (new_x, new_y)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
#[ignore]
    fn parse_test() {
        let input = "
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";
        let (m, a) = parse(input);
        println!("{} {a:?}", matrix_str(&m));
    }

    #[test]
    fn pt1_test() {
        let input = "
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";
        let (m, a) = parse(input);
        let got = pt_1(&m, &a);
        assert_eq!(got, 14);
    }

    #[test]
    fn pt2_test() {
        let input = "
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";
        let (m, a) = parse(input);
        let got = pt_2(&m, &a);
        assert_eq!(got, 34);
    }

    #[test]
    fn pt1_prod_test() {
        let input = fs::read_to_string("./inputs/day8.txt").unwrap();
        let (m, a) = parse(input.as_str());
        let got = pt_1(&m, &a);
        assert_eq!(got, 423);
    }

    #[test]
#[ignore]
    fn pt2_prod_test() {
        let input = fs::read_to_string("./inputs/day8.txt").unwrap();
        let (m, a) = parse(input.as_str());
        let got = pt_2(&m, &a);
        // 1287 is too low
        assert_eq!(got, 1);
    }
}
