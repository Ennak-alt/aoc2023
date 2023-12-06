use std::time::Duration;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn beats_record(hold: usize, duration: usize, record: usize) -> bool {
    let time_left = duration - hold;
    if time_left > 0 {
       return time_left*hold > record;
    }
    false
}

fn part1(input: &str) -> usize {
    dbg!(input);
    let v : Vec<Vec<usize>> = input.lines().map(|l| {
        let s = l.split(" ").filter(|s| *s != "").skip(1);
        s.map(|n| n.parse().unwrap()).collect()
    }).collect();
    dbg!(v.clone());
    let mut result = 1;
    for i in 0..v[0].len() {
        let race_dur = v[0][i];
        let record = v[1][i];

        let mut ways = 0;
        for i in 0..race_dur {
            if beats_record(i, race_dur, record) {
                ways += 1;
            }
        }
        result *= ways;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(288, part1(input));
    }
}
