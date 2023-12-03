use std::str;
fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> usize {
    let cma = input
        .lines()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut n = Vec::new();
    dbg!(cma.clone());
    for y in 0..(cma.len()) {
        let u = cma.get((y as i32 - 1) as usize);
        let m = cma.get(y).unwrap();
        let d = cma.get(y + 1);
        let mut not_this = -1;
        for x in 0..(cma[0].len()) {
            let c = m[x];
            if !c.is_ascii_digit() || x as i64 <= not_this {
                continue;
            }
            let mut v = Vec::new();
            if u != None {
                let u = u.unwrap();
                v.push(u.get((x as i32 - 1) as usize));
                v.push(u.get(x));
                v.push(u.get(x + 1));
            }
            if d != None {
                let d = d.unwrap();
                v.push(d.get((x as i32 - 1) as usize));
                v.push(d.get(x));
                v.push(d.get(x + 1));
            }
            v.push(m.get((x as i32 - 1) as usize));
            v.push(m.get(x));
            v.push(m.get(x + 1));
            if v.into_iter()
                .filter(|o| o.is_some())
                .map(|o| o.unwrap())
                .any(|c| {
                    dbg!(*c);
                    return *c != '.' && !(*c).is_ascii_alphanumeric();
                })
            {
                let mut str = String::from("");
                for x2 in (0..x).rev() {
                    if m[x2].is_ascii_digit() {
                        str = m[x2].to_string() + &str;
                    } else {
                        break;
                    }
                }
                for x3 in x..cma.len() {
                    if m[x3].is_ascii_digit() {
                        str = str + &m[x3].to_string();
                    } else {
                        if x3 - x != 0 {
                            not_this = x3 as i64;
                        }
                        break;
                    }
                }
                n.push(str.parse::<usize>().unwrap());
            }
        }
    }
    return n.into_iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(4361, part1(input));
    }
}
