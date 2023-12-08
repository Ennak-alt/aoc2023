use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = solution(input);
    dbg!(output);
}

fn solution(input: &str) -> usize {
    let start = "AAA";
    let end = "ZZZ";
    let v : Vec<&str> = input.split("\n\n").collect();
    let mut dirs = v[0].chars().cycle();
    let nodes : HashMap<&str, (&str, &str)> = v[1].lines().map(|l| {
        let n : Vec<&str> = l.split(" = ").collect();
        dbg!(n.clone());
        let n2 : Vec<&str> = n[1][1..(n[1].len()-1)].split(", ").collect();
        (n[0], (n2[0], n2[1]))
    }).collect();
    let mut current = start;
    let mut steps = 0;
    while current != end {
        let next_dir = dirs.next().unwrap();
        let (l, r) = nodes[current];
        if next_dir == 'R' {
            current = r;
        } else {
            current = l;
        }
        steps += 1;
    }
    steps
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(6, solution(input));
    }
}
