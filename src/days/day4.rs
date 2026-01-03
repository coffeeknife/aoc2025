use std::{fs::File, io::BufReader, io::BufRead};

pub fn run() {
    let file: File = File::open("inputs/day4.txt").expect("File opening error");
    let input: Vec<Vec<bool>> = BufReader::new(file).lines().map(|x| parse_line(&x.unwrap())).collect();

    let mut count: usize = 0;

    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] && count_neighbors(&input, (x, y)) < 4 {
                    count += 1;
            }
        }
    }

    println!("Part 1 Solution: {}", count);

    let mut input2 = input.clone();
    let mut removed_count: usize = 0;
    let mut y: usize = 0; let mut x: usize = 0;
    while y < input2.len() {
        while x < input2[0].len() {
            if input2[y][x] && count_neighbors(&input2, (x, y)) < 4 {
                input2[y][x] = false;
                removed_count += 1;
                x -= 1.min(x); y -= 1.min(y);
            } else {
                x += 1;
            }
        }
        y += 1; x = 0;
    } 
    println!("Part 2 Solution: {}", removed_count);

}

fn parse_line(line: &str) -> Vec<bool> {
    line.chars()
        .map(|x| if x == '@' { true } else { false })
        .collect()
}

fn count_neighbors(grid: &Vec<Vec<bool>>, pos: (usize, usize)) -> usize {
    let mut count: usize = 0;
    for y in ((pos.1 - 1.min(pos.1))..=(pos.1 + 1)).filter(|p| *p < grid.len()) {
        for x in ((pos.0 - 1.min(pos.0))..=(pos.0 + 1)).filter(|p| *p < grid[0].len()) {
            if !(x == pos.0 && y == pos.1) && grid[y][x] { 
                count += 1
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_neighbors() {
        let input: Vec<Vec<bool>> = vec![
            "..@@.@@@@.",
            "@@@.@.@.@@",
            "@@@@@.@.@@",
            "@.@@@@..@.",
            "@@.@@@@.@@",
            ".@@@@@@@.@",
            ".@.@.@.@@@",
            "@.@@@.@@@@",
            ".@@@@@@@@.",
            "@.@.@@@.@.",
        ]
        .iter()
        .map(|x| parse_line(x))
        .collect();

        let mut count: u32 = 0;

        for y in 0..input.len() {
            for x in 0..input[0].len() {
                if input[y][x] && count_neighbors(&input, (x, y)) < 4 {
                    count += 1;
                }
            }
        }

        assert_eq!(count, 13);
    }
}
