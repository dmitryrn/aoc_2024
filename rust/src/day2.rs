pub fn parse(input: &String) -> Vec<Vec<usize>> {
    input.lines()
        .filter(|x|!x.is_empty())
        .map(|l| l.split(' ').map(|x|x.parse::<usize>().unwrap()).collect::<Vec<_>>())
        .collect()
}

pub fn day2_pt1(list: Vec<Vec<usize>>) -> Vec<usize> {
    list.iter()
        .map(is_overall_valid)
        .collect()
}

pub fn day2_pt2(list: Vec<Vec<usize>>) -> Vec<usize> {
    fn is_valid(items: &Vec<usize>) -> usize {

        let full_valid = is_overall_valid(items) == 1;
        if full_valid {
            return 1
        }
        for i in 0..items.len() {
            let list_to_check = items
                .clone()
                .into_iter()
                .enumerate()
                .filter(|&(j, _)| j != i)
                .map(|(_, x)| x)
                .collect::<Vec<usize>>();
            if is_overall_valid(&list_to_check) == 1 {
                return 1
            }
        }
        return 0
    }

    list.iter()
        .map(is_valid)
        .collect()
}

fn is_overall_valid(items: &Vec<usize>) -> usize {
    let result: bool;
    if items[0] > items[1] {
        result = items.windows(2).all(|x| x[0] > x[1] && x[0].abs_diff(x[1]) <= 3);
    } else {
        result = items.windows(2).all(|x| x[0] < x[1] && x[0].abs_diff(x[1]) <= 3);
    }

    if result {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn test_day2_pt1() {
        let got = day2_pt1(parse(&"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
".to_string()));
        assert_eq!(got, vec![1, 0, 0, 0, 0, 1])
    }

    #[test]
    fn test_day2_pt2() {
        let got = day2_pt2(parse(&"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
".to_string()));
        assert_eq!(got, vec![1, 0, 0, 1, 1, 1])
    }

    #[test]
    fn test_day2_prod() {
        let file = fs::read_to_string("./inputs/day2.txt").unwrap();
        {
            let got = day2_pt1(parse(&file));
            assert_eq!(383, got.iter().sum::<usize>());
        }
        {
            let got = day2_pt2(parse(&file));
            assert_eq!(436, got.iter().sum::<usize>());
        }
    }
}

