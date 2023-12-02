use std::str;
fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> usize {
    input.lines().map(|s| s
            .split(": ")
            .nth(1)
            .unwrap()
            .split("; ")
            .collect::<Vec<&str>>()
            .iter()
            .map(|s3| 
                s3.split(", ").map(|s| {
                    let mut i = s.split(" ");
                    (i.next().unwrap().parse::<usize>().unwrap(), i.next().unwrap())
                }).collect::<Vec<(usize, &str)>>()
            )
            .collect::<Vec<Vec<(usize, &str)>>>()
    ).enumerate().map(|(i, game)| {
        let mut blue = 0;
        let mut green = 0;
        let mut red = 0;
        game.into_iter().for_each(|s| s.into_iter().for_each(|(n, b)| {
            match b {
                "blue" => if blue < n {blue = n},
                "red" => if red < n {red = n},
                _ => if green < n {green = n},
            }
        }));
        dbg!((red, green, blue));
        // only 12 red cubes, 13 green cubes, and 14 blue cubes
        if red <= 12 && green <= 13 && blue <= 14 {
            dbg!("Possible!");
            i+1
        } else {
            dbg!("Impossible!");
            0
        }
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(8, part1(input));
    }
}
