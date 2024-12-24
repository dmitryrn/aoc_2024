pub fn pt1(input: String) -> usize {
    let mut x_positions: Vec<(usize, usize)> = Vec::new();
    let matrix: Vec<Vec<char>> = input
        .lines()
        .filter(|l|!l.is_empty())
        .enumerate()
        .map(|(y, l)| 
            l.trim().chars().enumerate().map(|(x, c)| {
                if c == 'X' {
                    x_positions.push((x, y))
                }
                c
            }).collect()
        )
        .collect();

    let mut count = 0;
    for (x, y) in x_positions {
        count += star_find(&matrix, x, y);
    }

    count
}

// returns count of words
pub fn star_find(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    let mut count = 0;
    for positions in [
        [(Some(x), Some(y)), (x.checked_sub(1), Some(y)), (x.checked_sub(2), Some(y)), (x.checked_sub(3), Some(y))], // Left
        [(Some(x), Some(y)), (x.checked_add(1), Some(y)), (x.checked_add(2), Some(y)), (x.checked_add(3), Some(y))], // Right
        [(Some(x), Some(y)), (Some(x), y.checked_sub(1)), (Some(x), y.checked_sub(2)), (Some(x), y.checked_sub(3))], // Up
        [(Some(x), Some(y)), (Some(x), y.checked_add(1)), (Some(x), y.checked_add(2)), (Some(x), y.checked_add(3))], // Down
        [(Some(x), Some(y)), (x.checked_sub(1), y.checked_sub(1)), (x.checked_sub(2), y.checked_sub(2)), (x.checked_sub(3), y.checked_sub(3))], // Left-Up
        [(Some(x), Some(y)), (x.checked_add(1), y.checked_sub(1)), (x.checked_add(2), y.checked_sub(2)), (x.checked_add(3), y.checked_sub(3))], // Right-Up
        [(Some(x), Some(y)), (x.checked_sub(1), y.checked_add(1)), (x.checked_sub(2), y.checked_add(2)), (x.checked_sub(3), y.checked_add(3))], // Left-Down
        [(Some(x), Some(y)), (x.checked_add(1), y.checked_add(1)), (x.checked_add(2), y.checked_add(2)), (x.checked_add(3), y.checked_add(3))]  // Right-Down
    ] {
        let chars: Vec<Option<&char>> = positions
            .to_vec()
            .iter()
            .map(|(x, y)| {
                if x.is_none() || y.is_none() {
                    return None
                }
                let get = matrix.get(y.unwrap()).and_then(|row| row.get(x.unwrap()));
                get
            })
            .collect();
        if !chars.iter().all(|c|c.is_some()) {
            continue
        }
        let word: String = chars.iter().map(|c| *c.unwrap()).collect();
        if word != "XMAS" {
            continue
        }
        count += 1;
    }

    count
}

pub fn pt2(input: String) -> usize {
    let matrix: Vec<Vec<char>> = input
        .lines()
        .filter(|l|!l.is_empty())
        .map(|l| l.trim().chars().collect())
        .collect();

    let mut a_positions: Vec<(usize, usize)> = Vec::new();
    for y in 1..matrix.len()-1 {
        for x in 1..matrix[0].len()-1 {
            if matrix[y][x] == 'A' {
                a_positions.push((x, y));
            }
        }
    }

    let mut count = 0;
    for (x, y) in a_positions {
        if x_mas_search(&matrix, x, y) {
            count += 1
        }
    }

    count
}

pub fn x_mas_search(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    assert_eq!(matrix[y][x], 'A');
    // dbg!((search_x, search_y));

    let lu = matrix[y-1][x-1]; // left up
    let ld = matrix[y+1][x-1]; // left down
    let ru = matrix[y-1][x+1]; // right up
    let rd = matrix[y+1][x+1]; // right down

    [
        ('M', 'M', 'S', 'S'),
        ('S', 'M', 'M', 'S'),
        ('S', 'S', 'M', 'M'),
        ('M', 'S', 'S', 'M'),
    ].iter().any(|pattern| *pattern == (lu, ld, rd, ru))
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn pt1_test() {
        let got = pt1("
....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX
".to_string());
        assert_eq!(got, 18);

        assert_eq!(pt1(fs::read_to_string("./inputs/day4.txt").unwrap()), 2613);
    }

    #[test]
    fn pt2_test() {
        let got = pt2("
.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........
".to_string());
        assert_eq!(got, 9);

        // 1968 is too high
        assert_eq!(pt2(fs::read_to_string("./inputs/day4.txt").unwrap()), 1905);
    }
}
