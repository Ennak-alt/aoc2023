use std::str;
fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> usize {
    let parsed: Vec<&str> = input.lines().collect();
    let mut numbers: Vec<usize> = Vec::new();
    parsed.iter().for_each(|s| {
        let s1 = s.to_string();
        let mut s2 = s1.as_bytes().iter();
        let mut bytes = Vec::<u8>::new();
        dbg!(s);
        loop {
            let bo = s2.next();
            if let Some(b) = bo {
                dbg!(b);
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
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        assert_eq!(142, part1(input));
    }
}
