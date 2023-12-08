use std::time::Duration;

fn main() {
    let input = include_str!("./input1.txt");
    let output = solution(input);
    dbg!(output);
}

fn cardval(c: char) -> usize {
    dbg!(c);
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => c.to_digit(10).unwrap() as usize,
    }
}

fn same(cs: &[char]) -> bool {
    cs.into_iter().all(|c| *c == cs[0])
}

fn val(cards: &Vec<char>) -> usize {
    let mut cp = cards.clone();
    cp.sort();
    if same(&cp[0..]) {
        7
    } else if same(&cp[0..=3]) || same(&cp[1..=4]) {
        6
    } else if (same(&cp[0..=2]) && same(&cp[3..])) || (same(&cp[0..=1]) && same(&cp[2..])) {
        5
    } else if same(&cp[0..=2]) || same(&cp[1..=3]) || same(&cp[2..]) {
        4
    } else if (cp[0] == cp[1] && cp[2] == cp[3])
        || (cp[1] == cp[2] && cp[3] == cp[4])
        || (cp[0] == cp[1] && cp[3] == cp[4])
    {
        3
    } else if cp[0] == cp[1] || cp[1] == cp[2] || cp[2] == cp[3] || cp[3] == cp[4] {
        2
    } else {
        1
    }
}

fn solution(input: &str) -> usize {
    let mut cards = input
        .lines()
        .map(|l| {
            let v: Vec<&str> = l.split_ascii_whitespace().collect();
            (
                v[0].chars().collect::<Vec<char>>(),
                v[1].parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<(Vec<char>, usize)>>();
    cards.sort_by(|a, b| {
        let v1 = val(&a.0);
        let v2 = val(&b.0);
        if v1 == v2 {
            let cc1: Vec<usize> = a.clone().0.into_iter().map(|c| cardval(c)).collect();
            let cc2: Vec<usize> = b.clone().0.into_iter().map(|c| cardval(c)).collect();
            for i in 0..cc1.len() {
                if cc1[i] != cc2[i] {
                    return cc1[i].cmp(&cc2[i]);
                }
            }
        }
        return v1.cmp(&v2);
    });
    cards
        .into_iter()
        .enumerate()
        .inspect(|c| {
            println!("{:?}", c.clone());
        })
        .fold(0, |acc: usize, (n, (_, bid))| acc + (n + 1) * bid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(6440, solution(input));
    }
}
