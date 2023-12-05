use std::{
    borrow::BorrowMut,
    collections::{HashSet, VecDeque},
    str,
};
fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug, Copy, Clone)]
struct Range {
    des_start: usize,
    src_start: usize,
    range_length: usize,
}

impl Range {
    pub fn src_end(&self) -> usize {
        self.src_start + self.range_length
    }
}

fn part1(input: &str) -> usize {
    let mappings: Vec<&str> = input.split("\n\n").collect();
    let mut seeds_it = mappings[0]
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap());
    let mut seeds: Vec<(usize, usize)> = Vec::new();
    while let Some(seed) = seeds_it.next() {
        seeds.push((seed, seeds_it.next().unwrap()))
    }
    dbg!(seeds.clone());
    let mappings: Vec<Vec<Range>> = mappings[1..]
        .into_iter()
        .map(|map| {
            let mut mapping: Vec<Range> = Vec::new();
            map.lines().skip(1).for_each(|s| {
                let nums: Vec<usize> = s.split(" ").map(|s| s.parse::<usize>().unwrap()).collect();
                mapping.push(Range {
                    des_start: nums[0],
                    src_start: nums[1],
                    range_length: nums[2],
                });
            });
            mapping
        })
        .collect();
    for i in 0..mappings.len() {
        dbg!(seeds.clone());
        seeds.sort_by_key(|k| k.0);
        let mut seeds2 = vec![seeds[0]];
        for j in 1..seeds.len() {
            let (seed, range) = seeds2.pop().unwrap();
            let (seed2, range2) = seeds[j];
            if seed + range >= seed2 && seed2 + range2 >= seed + range {
                seeds2.push((seed, (seed2 + range2) - seed));
            } else {
                seeds2.push((seed, range));
                seeds2.push(seeds[j]);
            }
        }
        seeds = seeds2
            .into_iter()
            .filter(|(_, range)| *range != 0)
            .collect();
        dbg!(seeds.clone());
        let mut new_seed_list = Vec::new();
        let m = mappings[i].clone();
        seeds.into_iter().for_each(|s| {
            let mut v = vec![s];
            for j in 0..m.len() {
                let mut vc = Vec::new();
                while let Some((seed, range)) = v.pop() {
                    let seed_end = seed + range;
                    if m[j].src_start <= seed && seed_end <= m[j].src_end() {
                        // Seed total overlap
                        dbg!(("total", seed, seed_end, m[j].src_start,  m[j].src_end()));
                        new_seed_list.push((m[j].des_start + (seed - m[j].src_start), range));
                        dbg!(new_seed_list.clone().into_iter().last().unwrap());
                    } else if seed < m[j].src_start
                        && seed_end >= m[j].src_start
                        && seed_end <= m[j].src_end()
                    {
                        dbg!(("end", seed, seed_end, m[j].src_start,  m[j].src_end()));
                        // Seed end overlap
                        new_seed_list.push((m[j].des_start, seed_end - m[j].src_start));
                        vc.push((seed, (m[j].src_start - 1) - seed+1));
                        dbg!(new_seed_list.clone().into_iter().last().unwrap());
                    } else if m[j].src_start <= seed
                        && m[j].src_end() < seed_end
                        && seed <= m[j].src_end()
                    {
                        dbg!(("start", seed, range, m[j].src_start,  m[j].src_end()));
                        // Seed start overlap
                        new_seed_list.push((
                            m[j].des_start + (seed - m[j].src_start),
                            m[j].src_end() - seed,
                        ));
                        vc.push((m[j].src_end(), seed_end - (m[j].src_end() + 1)+1));
                        dbg!(new_seed_list.clone().into_iter().last().unwrap());
                    } else if m[j].src_start > seed && m[j].src_start < seed_end && m[j].src_end() > seed && seed_end > m[j].src_end() { 
                        dbg!(("middle", seed, seed_end, m[j].src_start,  m[j].src_end()));
                        // Seed middle overlap
                        new_seed_list.push((m[j].des_start, m[j].range_length));
                        vc.push((m[j].src_end(), seed_end - (m[j].src_end() + 1)+1));
                        vc.push((seed, (m[j].src_start - 1) - seed+1));
                        dbg!(new_seed_list.clone().into_iter().last().unwrap());
                    } else {
                        // No overlap
                        vc.push((seed, range));
                        dbg!(("nothing", seed, seed_end, m[j].src_start,  m[j].src_end())); 
                    }
                }
                v = vc
            }
            new_seed_list.append(&mut v); // Append the rest on that could not be mapped
        });
        seeds = new_seed_list;
    }
    seeds.sort_by_key(|k| k.0);
    dbg!(seeds.clone());
    seeds[0].0
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
        assert_eq!(46, part1(input));
    }
}
