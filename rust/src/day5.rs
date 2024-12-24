use std::cmp::Ordering;

pub fn parse(input: String) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let rules: Vec<(usize, usize)> = input
        .lines()
        .skip_while(|l| l.is_empty())
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let mut spl = l.split('|');
            (spl.next().unwrap().parse().unwrap(), spl.next().unwrap().parse().unwrap())
        })
        .collect();

    let updates: Vec<Vec<usize>> = input
        .lines()
        .filter(|l| l.contains(','))
        .map(|l| l.split(',').map(|n| n.parse::<usize>().unwrap()).collect())
        .collect();

    (rules, updates)
}

pub fn soln(rules: Vec<(usize, usize)>, updates: Vec<Vec<usize>>) -> Vec<(Vec<usize>, Vec<usize>)> {
    updates.iter().map(|pages| {
        let relevant_rules: Vec<_> = rules.iter()
            .filter(|(a, b)| pages.contains(a) || pages.contains(b)).collect();
        let mut pages_sorted = pages.clone();
        pages_sorted.sort_unstable_by(|p1, p2| {
            if relevant_rules.iter().find(|(a, b)| a == p1 && b == p2).is_some() {
                return Ordering::Less
            }
            if relevant_rules.iter().find(|(a, b)| b == p1 && a == p2).is_some() {
                return Ordering::Greater
            }
            Ordering::Equal
        });
        (pages.clone(), pages_sorted)
    }).collect()
}

pub fn pt1(input: String) -> usize {
    let (rules, updates) = parse(input);
    let lists = soln(rules, updates);
    lists.iter().map(|(update, update_sorted)| {
        if update == update_sorted {
            return update[update.len()/2]
        }
        0
    }).sum()
}

pub fn pt2(input: String) -> usize {
    let (rules, updates) = parse(input);
    let lists = soln(rules, updates);
    lists.iter().map(|(update, update_sorted)| {
        if update != update_sorted {
            return update_sorted[update_sorted.len()/2]
        }
        0
    }).sum()
}


#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn pt1_test() {
        let got = pt1("
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
".to_string());
        assert_eq!(got, 143);
    }

    #[test]
    fn pt1_prod() {
        assert_eq!(pt1(fs::read_to_string("./inputs/day5.txt").unwrap()), 6612);
    }

    #[test]
    fn test_pt2() {
        let got = pt2("
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
".to_string());
        assert_eq!(got, 123);
    }

    #[test]
    fn pt2_prod() {
        assert_eq!(pt2(fs::read_to_string("./inputs/day5.txt").unwrap()), 4944);
    }
}
