
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


fn dist(duration : usize, hold: usize) -> usize {
    let time_left = duration - hold;
    if time_left > 0 {
        return time_left*hold;
    }
    0
}

fn part1(input: &str) -> usize {
    dbg!(input);
    let v : Vec<usize> = input.lines().map(|l| {
        let s = l.split(" ").filter(|s| *s != "").skip(1);
        let mut s1 = String::from("");
        s.for_each(|s2| s1 += s2);
        s1.parse().unwrap()
    }).collect();
    dbg!(v.clone());
    let race_dur = v[0];
    let record = v[1];
    let mut ways = 0;
    for i in 0..race_dur {
        if beats_record(i, race_dur, record) {
            ways += 1;
        }
    }
    ways
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(71503, part1(input));
    }
}
