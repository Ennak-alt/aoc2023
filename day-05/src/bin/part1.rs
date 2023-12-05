use std::{collections::HashSet, str};
fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

type Mapping = Vec<Range>;

#[derive(Debug, Copy, Clone)]
struct Range {
    des_start: usize,
    src_start: usize,
    range_length: usize,
}

fn part1(input: &str) -> usize {
    let mappings: Vec<&str> = input.split("\n\n").collect();
    let seeds: Vec<usize> = mappings[0]
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let mappings : Vec<Vec<Range>> = mappings[1..].into_iter().map(|map| {
        let mut mapping : Vec<Range> = Vec::new();
        map.lines().skip(1).for_each(|s| {
            let nums : Vec<usize> = s.split(" ").map(|s| s.parse::<usize>().unwrap()).collect();
            mapping.push(Range { des_start: nums[0], src_start: nums[1], range_length: nums[2] });
        });
        mapping
    }).collect();
    seeds.into_iter().map(|seed| {
        let mut s = seed;
        for i in 0..mappings.len() {
            let mapping = mappings[i].clone();
            let mut n = None;
            for x in 0..mapping.len() {
                let map = mapping[x];
                if s >= map.src_start && s <= map.src_start+map.range_length {
                    n = Some(map.des_start + (s - map.src_start));
                    break;
                }
            }
            if n.is_some() {
                s = n.unwrap();
            }
        }
        s
    }).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(35, part1(input));
    }
}
