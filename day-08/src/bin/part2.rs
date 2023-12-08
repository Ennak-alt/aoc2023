use std::collections::HashMap;
use num::integer::lcm;

fn main() {
    let input = include_str!("./input1.txt");
    let output = solution(input);
    dbg!(output);
}

fn solution(input: &str) -> usize {
    let v: Vec<&str> = input.split("\n\n").collect();
    let nodes: HashMap<&str, (&str, &str)> = v[1]
        .lines()
        .map(|l| {
            let n: Vec<&str> = l.split(" = ").collect();
            dbg!(n.clone());
            let n2: Vec<&str> = n[1][1..(n[1].len() - 1)].split(", ").collect();
            (n[0], (n2[0], n2[1]))
        })
        .collect();
    let starting_nodes : Vec<(&str, (&str, &str))> = nodes
        .clone()
        .into_iter()
        .filter(|(s1, _)| s1.chars().nth(2).unwrap() == 'A')
        .collect();
    let m : Vec<usize> = starting_nodes.into_iter().map(|(n, _)| {
        let mut step = 0;
        let mut total_steps = 0;
        let mut dirs = v[0].chars().cycle();
        let mut current = n;
        let mut hm : HashMap<String, (Vec<usize>, Vec<usize>)> = HashMap::new();
        
        loop {
            if current.chars().last().unwrap() == 'Z' {
                if hm.contains_key(current) {
                    if hm[current].0.contains(&step) {
                        return hm[current].1[0];
                    } else {
                        let (step_v, total_v) = hm.get_mut(current).unwrap();
                        step_v.push(step);
                        total_v.push(total_steps);
                    }
                } else {
                    hm.insert(current.to_string(), (vec![step], vec![total_steps]));
                }
            }
            let next_dir = dirs.next().unwrap();
            let (l, r) = nodes[current];
            if next_dir == 'R' {
                current = r;
            } else {
                current = l;
            }
            step = (step + 1) % v[0].len();
            total_steps += 1;
        }
    }).collect();

    m.into_iter().fold(1, |acc, x| lcm(acc, x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(6, solution(input));
    }
}
