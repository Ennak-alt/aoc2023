use core::num;
use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = solution(input);
    dbg!(output);
}

fn solution(input: &str) -> i128 {
    let numbers: Vec<Vec<i128>> = input
        .lines()
        .map(|l| l.split(" ").map(|n| n.parse().unwrap()).collect())
        .collect();
    dbg!(numbers.clone());
    numbers.into_iter().map(|v| {
        dbg!(v.clone());
        let mut vc = v.clone();
        let mut lasts : Vec<i128> = vec![*v.first().unwrap()];
        loop {
            let mut vc2 = Vec::new();
            let mut zeros = 0;
            for i in 0..vc.len()-1 {
                vc2.push(vc[i+1] - vc[i]);
                if vc[i+1] - vc[i] == 0 {
                    zeros += 1;
                }
            }
            dbg!(vc2.clone());
            lasts.push(*vc2.first().unwrap());
            vc = vc2.clone();
            if zeros == vc2.len() {
                break;
            }
        }
        dbg!(lasts.clone());
        dbg!(lasts.clone().into_iter().rev().fold(0, |acc, x| acc - x));
        lasts.clone().into_iter().rev().fold(0, |acc, x| x - acc)
    }).sum::<i128>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(2, solution(input));
    }
}
