use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn run() {
    let input: Vec<i32> = parse_input("inputs/day1.txt");
    println!("Part 1: {}", pt1(&input));
    println!("Part 2: {}", pt2(&input));
}

fn pt1(input: &Vec<i32>) -> u32 {
    let mut position: i32 = 50;
    let mut zero_count: u32 = 0;
    for turn in input {
        position = turn_abs(position + turn);
        if position == 0 { zero_count += 1 }
    }
    zero_count
}

fn pt2(input: &Vec<i32>) -> u32 {
    let mut position: i32 = 50;
    let mut zero_count: u32 = 0;
    for turn in input {
        let last_pos: i32 = position;
        let pos_new: i32 = position + turn;
        position = turn_abs(pos_new);

        if last_pos != 0 && position == 0 { zero_count += (turn.abs() / 100) as u32 + 1 }
        else if last_pos == 0 { zero_count += (turn.abs() / 100) as u32 }
        else if pos_new != position { zero_count += ((pos_new - position).abs() / 100) as u32 }
    }
    zero_count
}

fn turn_abs(input: i32) -> i32 {
    if input < 0 { input - (input / 100 - 1) * 100 } else { input % 100 }
}

fn parse_input(filename: &str) -> Vec<i32> {
    let file: File = File::open(filename).expect("File opening error");
    BufReader::new(file).lines().map(|x| -> i32 {
        let binding = x.unwrap(); let mut line = binding.chars();
        let dir: i32 = match line.next().unwrap() { 'L' => -1, 'R' => 1, _ => panic!() };
        line.flat_map(|y| y.to_digit(10)).fold(0, |acc, elem| acc * 10 + (elem as i32)) * dir
     }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(pt1(&vec![-68,-30,48,-5,60,-55,-1,-99,14,-82]), 3);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(pt2(&vec![-68,-30,48,-5,60,-55,-1,-99,14,-82]), 6);
        assert_eq!(pt2(&vec![160]), 2);
        assert_eq!(pt2(&vec![50,-1]), 1);
        assert_eq!(pt2(&vec![-50,1]), 1);
        assert_eq!(pt2(&vec![50, -100]), 2);
        assert_eq!(pt2(&vec![1000,-149,-1,1,-2,1,-1,2,99]), 16);
        assert_eq!(pt2(&vec![-151]), 2);
    }
}