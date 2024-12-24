use regex::Regex;

pub fn parse_mul(str: &str) -> (usize, usize) {
    let replace = str.replace("mul(", "");
    let replace = replace.replace(")", "");
    let mut spl = replace.split(',');
    (
        spl.next().unwrap().parse().unwrap(), 
        spl.next().unwrap().parse().unwrap(),
    )
}

pub fn pt1(input: String) -> Vec<(usize, usize)> {
    let re_muls = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let muls = re_muls.find_iter(input.as_str()).collect::<Vec<_>>();

    muls.iter().map(|mul| parse_mul(mul.as_str())).collect()
}

pub fn pt2(input: String) -> Vec<(usize, usize)> {
    let re_muls = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let mut muls = re_muls.find_iter(input.as_str()).collect::<Vec<_>>();

    let re_commands = Regex::new(r"(do\(\)|don't\(\))").unwrap();
    let mut commands = re_commands.find_iter(input.as_str()).collect::<Vec<_>>();

    commands.append(&mut muls);
    commands.sort_by(|a, b|a.start().cmp(&b.start()));

    let mut capturing = true;
    commands.iter().filter(|m| {
        match m.as_str() {
            str if str.starts_with("mul") => capturing,
            "do()" => {
                capturing = true;
                false
            }
            "don't()" => {
                capturing = false;
                false
            }
            str => panic!("Unexpected command: {}", str),
        }
    }).map(|mul| parse_mul(mul.as_str())).collect()
}

pub fn answer(input: Vec<(usize, usize)>) -> usize {
    input.iter().map(|(a, b)| a * b).sum()
}


#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn pt1_test() {
        let got = pt1("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string());
        assert_eq!(got, vec![(2,4),(5,5),(11,8),(8,5)]);
        assert_eq!(answer(got), 161);

        assert_eq!(answer(pt1(fs::read_to_string("./inputs/day3.txt").unwrap())), 161289189);
    }

    #[test]
    fn pt2_test() {
        let got = pt2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string());
        assert_eq!(got, vec![(2,4),(8,5)]);

        assert_eq!(answer(pt2(fs::read_to_string("./inputs/day3.txt").unwrap())), 83595109);
    }
}
