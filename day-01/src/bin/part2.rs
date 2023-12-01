use std::str;
fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> usize {
    let parsed: Vec<&str> = input.lines().collect();
    let mut numbers: Vec<usize> = Vec::new();
    parsed.iter().for_each(|s| {
        let s1 = s.to_string();
        let mut s2 = s1.as_bytes().iter();
        let mut bytes = Vec::<u8>::new();
        dbg!(s);
        loop {
            let b2 : Vec<&u8> = s2.clone().collect();
            let bo = s2.next();
            if let Some(b) = bo {
                dbg!(b);
                let b3 = b2.into_iter().map(|b| { *b }).collect::<Vec<u8>>();
                let str = str::from_utf8(&b3).unwrap();
                if str.len() >= 4 && str[0..4] == "zero".to_string() {
                    bytes.push(b'0');
                } else if str.len() >= 3 && str[0..3] == "one".to_string() {
                    bytes.push(b'1');
                } else if str.len() >= 3 && str[0..3] == "two".to_string() {
                    bytes.push(b'2');
                } else if str.len() >= 5 && str[0..5] == "three".to_string() {
                    bytes.push(b'3');
                } else if str.len() >= 4 && str[0..4] == "four".to_string() {
                    bytes.push(b'4');
                } else if str.len() >= 4 && str[0..4] == "five".to_string() {
                    bytes.push(b'5');
                } else if str.len() >= 3 && str[0..3] == "six".to_string() {
                    bytes.push(b'6');
                } else if str.len() >= 5 && str[0..5] == "seven".to_string() {
                    bytes.push(b'7');
                } else if str.len() >= 5 && str[0..5] == "eight".to_string() {
                    bytes.push(b'8');
                } else if str.len() >= 4 && str[0..4] == "nine".to_string() {
                    bytes.push(b'9');
                }
                if b.is_ascii_digit() {
                    bytes.push(*b);
                }
            } else {
                break;
            }
        }
        dbg!(bytes.clone());
        numbers.push(str::from_utf8(&vec![bytes[0], bytes[bytes.len()-1]]).unwrap().parse().unwrap());
    });
    dbg!(numbers.clone());
    return numbers.iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        assert_eq!(281, part2(input));
    }
}
