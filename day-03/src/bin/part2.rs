use std::str;
fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn get_num(m: &Vec<char>, x: usize, end: usize) -> usize {
    let mut str = String::from("");
    for x2 in (0..x).rev() {
        if m[x2].is_ascii_digit() {
            str = m[x2].to_string() + &str;
        } else {
            break;
        }
    }
    for x3 in x..end {
        if m[x3].is_ascii_digit() {
            str = str + &m[x3].to_string();
        } else {
            break;
        }
    }
    dbg!(str.clone());
    str.parse::<usize>().unwrap()
}

fn part2(input: &str) -> usize {
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
        for x in 0..(cma[0].len()) {
            let c = m[x];
            if c != '*' {
                continue;
            }
            let mut state = [[None; 3]; 3];
            if u != None {
                let u = u.unwrap();
                state[0][0] = u.get((x as i32 - 1) as usize);
                state[0][1] = u.get(x);
                state[0][2] = u.get(x + 1);
            }
            if d != None {
                let d = d.unwrap();
                state[2][0] = d.get((x as i32 - 1) as usize);
                state[2][1] = d.get(x);
                state[2][2] = d.get(x + 1);
            }
            state[1][0] = m.get((x as i32 - 1) as usize);
            state[1][1] = m.get(x);
            state[1][2] = m.get(x + 1);
            fn is_sn(c : Option<&char>) -> bool {
                if let Some(c2) = c {
                    c2.is_ascii_digit()
                } else {
                    false
                }
            }
            dbg!(state);

            let mut v = Vec::new();
            let newy = y as i64 - 1;
            for y2 in 0..3 {
                if is_sn(state[y2][0]) || is_sn(state[y2][1]) || is_sn(state[y2][2]) {
                    if is_sn(state[y2][0]) && is_sn(state[y2][2]) && !is_sn(state[y2][1]) {
                        v.push(get_num(cma.get((newy + y2 as i64) as usize).unwrap(), x-1, cma[0].len()));
                        v.push(get_num(cma.get((newy + y2 as i64) as usize).unwrap(), x+1, cma[0].len()));
                    } else {
                        if is_sn(state[y2][0]) {
                            v.push(get_num(cma.get((newy + y2 as i64) as usize).unwrap(), x-1, cma[0].len()));
                        } else if is_sn(state[y2][1]) {
                            v.push(get_num(cma.get((newy + y2 as i64) as usize).unwrap(), x, cma[0].len()));
                        } else {
                            v.push(get_num(cma.get((newy + y2 as i64) as usize).unwrap(), x+1, cma[0].len()));     
                        }
                    }
                }
            }
            
            dbg!(v.clone());
            if v.len() == 2 {
                n.push(v[0] * v[1]);
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
        assert_eq!(467835, part2(input));
    }
}
