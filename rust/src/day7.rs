// 1 2 3
// 2 1 3              1 3 2
// 1 2 3  2 3 1       3 1 2  1 2 3

// [+ +] +
// * + +  + * +  * * +   + * +  + + *  + * *
//

use std::collections::HashSet;

pub fn permutations_handy(vec: Vec<char>) -> Vec<Vec<char>> {
    let mut result = HashSet::new();
    permutations(vec, 0, &mut result);
    result.into_iter().collect()
}

pub fn permutations(vec: Vec<char>, i: usize, result: &mut HashSet<Vec<char>>) {
    if vec.len() == 1 {
        result.extend(vec![vec!['+'], vec!['*']]);
        return
    }

    // println!("{vec:?}");
    for i in i..(vec.len() - 1) {
        let j = i + 1;
        result.insert(vec.clone());

        let to_process: Vec<_> = {
            let mut vec1 = vec.clone(); 
            vec1[i] = '*'; 
            vec1[j] = '+'; 

            let mut vec2 = vec.clone(); 
            vec2[i] = '+'; 
            vec2[j] = '*'; 

            let mut vec3 = vec.clone(); 
            vec3[i] = '*'; 
            vec3[j] = '*'; 

            let mut vec4 = vec.clone(); 
            vec4[i] = '+'; 
            vec4[j] = '+'; 

            vec![vec1, vec2, vec3, vec4].into_iter().filter(|v| *v != vec).collect()
        };
        result.extend(to_process.clone());

        for vec in to_process {
            permutations(vec, j, result);
        }
    }
}

pub fn parse(input: &str) -> Vec<(usize, Vec<usize>)> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let spl: Vec<&str> = l.trim().split(": ").collect();
            assert_eq!(spl.len(), 2);
            (
                spl[0].parse().unwrap(),
                spl[1].split(' ').map(|n| n.parse().unwrap()).collect(),
            )
        })
        .collect()
}

pub fn pt1(input: Vec<(usize, Vec<usize>)>) -> usize {
    let items: Vec<_> = input
        .into_iter()
        .map(|(answer, nums)| {
            (answer, nums.clone(), single(answer, nums))
        })
        .collect();
    // println!("{:?}", items.clone());

    items.into_iter().filter_map(|(answer, _, valid)| if valid { Some(answer) } else { None }).sum()
}

pub fn single(answer: usize, nums: Vec<usize>) -> bool {
    let signs = {
        let mut vec = Vec::new();
        nums.iter().take(nums.len() - 1).for_each(|_| vec.push('+'));
        vec
    };
    let permutations = permutations_handy(signs);

    println!("{:?} {:?} {:?}", answer, nums.clone(), permutations.len());

    let valid = permutations.iter().any(|signs| {
        answer == calc(&nums, signs)
    });

    // println!("{valid}");
    // println!();

    valid
}

pub fn calc(nums: &Vec<usize>, signs: &Vec<char>) -> usize {
    let mut i = 0;
    nums.clone().into_iter().reduce(|acc, n| {
        let sign = signs[i];
        i += 1;
        if sign == '+' {
            acc + n
        } else {
            acc * n
        }
    }).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn calc_test_1() {
        assert_eq!(calc(&vec![10, 19], &vec!['+']), 29);
        assert_eq!(calc(&vec![10, 19], &vec!['*']), 190);

    }
    #[test]
    fn calc_test_2() {
        assert_eq!(calc(&vec![81, 40, 27], &vec!['+', '*']), 3267);
    }

    #[test]
    fn permutations_small() {
        let mut got = permutations_handy(vec!['+', '+']);
        got.sort();
        assert_eq!(got, vec![vec!['*', '*'], vec!['*', '+'], vec!['+', '*'], vec!['+', '+']]);
    }

    #[test]
    fn permutations_test_test() {
        let mut got = permutations_handy(vec!['+', '+', '+']);
        got.sort_by(|a, b| b.cmp(a));
        assert_eq!(got, vec![
            vec!['+', '+', '+'], 
            vec!['+', '+', '*'], 
            vec!['+', '*', '+'], 
            vec!['+', '*', '*'],
            vec!['*', '+', '+'], 
            vec!['*', '+', '*'],
            vec!['*', '*', '+'],
            vec!['*', '*', '*'],
        ]);
    }

    #[test]
    fn test_single_1() {
       assert_eq!(single(190, vec![10, 19]), true);
    }

    #[test]
    fn test_single_2() {
       assert_eq!(single(3267, vec![81, 40, 27]), true);
    }

    #[test]
    fn test_single_3() {
       assert_eq!(single(2141581321406, vec![7,3,49,65,73,383,82]), false);
    }


    #[test]
    fn pt1_test() {
        let input = parse("
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
");
        let ans = pt1(input);
        assert_eq!(ans, 3749)
    }

    #[test]
#[ignore]
    fn pt1_prod_test() {
        assert_eq!(pt1(parse(fs::read_to_string("./inputs/day7.txt").unwrap().as_str())), 6231007345478);
    }
}
