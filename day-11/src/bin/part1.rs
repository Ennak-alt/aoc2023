use core::num;
use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = solution(input);
    dbg!(output);
}

fn shortest(
    (x, y): (usize, usize),
    (x2, y2): (usize, usize),
    empty_x: &Vec<usize>,
    empty_y: &Vec<usize>,
) -> usize {
    let mut steps = 0;
    if x2 >= x {
        for i in (x+1)..=x2 {
            if empty_x.contains(&i) {
                steps += 1;
            }
            steps += 1;
        }
    } else {
        for i in x2..x {
            if empty_x.contains(&i) {
                steps += 1;
            }
            steps += 1;
        }
    }
    for i in (y+1)..=y2 {
        if empty_y.contains(&i) {
            steps += 1;
        }
        steps += 1;
    }
    steps
}

fn solution(input: &str) -> usize {
    let mut v = Vec::new();
    let mut map = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars().enumerate().for_each(|(x, s)| {
                if s == '#' {
                    v.push((x, y))
                }
            });
            l.chars().collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();
    let mut empty_x = Vec::new();
    let mut empty_y = Vec::new();
    for z in 0..map.len() {
        if !v.clone().into_iter().any(|(x, _)| z == x) {
            empty_x.push(z);
        }
        if !v.clone().into_iter().any(|(_, y)| z == y) {
            empty_y.push(z);
        }
    }
    let mut vv = Vec::new();
    for i in 0..v.len() - 1 {
        for j in (i+1)..v.len() {
            let f = v[i];
            let s = v[j];
            vv.push(shortest(f, s, &empty_x, &empty_y));
        }
    }
    vv.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(374, solution(input));
    }
}
