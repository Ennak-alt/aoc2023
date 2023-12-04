use std::{str, collections::HashSet};
fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> usize {
    let cardwins = input.lines().map(|s| s.split(": ").nth(1).unwrap()).map(|s| {
        let mut split = s.split(" | ");
        (split.next().unwrap().split(" ").filter(|s| *s != "").map(|n| n.parse().unwrap()).collect(), 
        split.next().unwrap().split(" ").filter(|s| *s != "").map(|n| n.parse().unwrap()).collect())
    }).collect::<Vec<(HashSet<usize>, Vec<usize>)>>().into_iter().map(|c| {
        let (win, num) = c;
        let mut p = 0;
        num.into_iter().for_each(|n| {
            if win.contains(&n) {
                p += 1
            }
        });
        p
    }).enumerate().collect::<Vec<(usize,usize)>>();
    let mut previous: Vec<i64> = vec![-1; cardwins.len()];
    // Dynamic programming ftw
    cardwins.clone().into_iter().rev().for_each(|(i, n)| {
        let mut start = vec![(i, n)];
        let mut local = 0;
        while let Some((ii, ni)) = start.pop() {
            if previous[ii] != -1 {
                local += previous[ii] as usize
            } else {
                local += cardwins[(ii+1)..(ii+1+ni)].len();
                start.extend_from_slice(&cardwins[(ii+1)..(ii+1+ni)]);
            }
        }
        previous[i] = local as i64;
    });
    previous.into_iter().sum::<i64>() as usize + cardwins.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(30, part2(input));
    }
}
