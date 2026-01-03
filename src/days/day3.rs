use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run() {
    let file: File = File::open("inputs/day3.txt").expect("File opening error");
    let input: Vec<Vec<u64>> = BufReader::new(file)
        .lines()
        .map(|x| {
            x.unwrap()
                .chars()
                .map(|y| y.to_digit(10).unwrap() as u64)
                .collect()
        })
        .collect();
    println!(
        "Part 1: {}",
        input.iter().map(|x| joltage(x.clone())).sum::<u64>()
    );
    println!(
        "Part 2: {}",
        input.iter().map(|x| crazy_joltage(x.clone())).sum::<u64>()
    );
}

fn joltage(bank: Vec<u64>) -> u64 {
    let (max, pos): (u64, usize) = max_with_reserve(&bank, 1);
    bank.iter().skip(pos + 1).max().unwrap() + 10 * max
}

fn crazy_joltage(bank: Vec<u64>) -> u64 {
    let mut joltage: u64 = 0;
    let mut bank_mut = bank.clone();
    for n in (0..=11).rev() {
        let (max, pos) = max_with_reserve(&bank_mut, n);
        joltage = joltage * 10 + max;
        bank_mut = bank_mut
            .iter()
            .skip(pos + 1)
            .map(|x| x.clone())
            .collect_vec();
    }
    joltage
}

fn max_with_reserve(bank: &Vec<u64>, reserved: usize) -> (u64, usize) {
    // value, index
    let max: u64 = *bank.iter().take(bank.len() - reserved).max().unwrap();
    (max, bank.iter().position(|x| *x == max).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_joltage() {
        let bat1: Vec<u64> = "987654321111111"
            .chars()
            .map(|x| x.to_digit(10).unwrap() as u64)
            .collect();
        let bat2: Vec<u64> = "811111111111119"
            .chars()
            .map(|x| x.to_digit(10).unwrap() as u64)
            .collect();
        let bat3: Vec<u64> = "234234234234278"
            .chars()
            .map(|x| x.to_digit(10).unwrap() as u64)
            .collect();
        let bat4: Vec<u64> = "818181911112111"
            .chars()
            .map(|x| x.to_digit(10).unwrap() as u64)
            .collect();

        assert_eq!(joltage(bat1), 98);
        assert_eq!(joltage(bat2), 89);
        assert_eq!(joltage(bat3), 78);
        assert_eq!(joltage(bat4), 92);
    }

    #[test]
    fn test_crazy_joltage() {
        let bat1: Vec<u64> = "987654321111111"
            .chars()
            .map(|x| x.to_digit(10).unwrap() as u64)
            .collect();
        let bat2: Vec<u64> = "811111111111119"
            .chars()
            .map(|x| x.to_digit(10).unwrap() as u64)
            .collect();
        let bat3: Vec<u64> = "234234234234278"
            .chars()
            .map(|x| x.to_digit(10).unwrap() as u64)
            .collect();
        let bat4: Vec<u64> = "818181911112111"
            .chars()
            .map(|x| x.to_digit(10).unwrap() as u64)
            .collect();

        assert_eq!(crazy_joltage(bat1), 987654321111);
        assert_eq!(crazy_joltage(bat2), 811111111119);
        assert_eq!(crazy_joltage(bat3), 434234234278);
        assert_eq!(crazy_joltage(bat4), 888911112111);
    }
}
