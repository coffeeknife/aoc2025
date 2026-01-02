use itertools::Itertools;
use std::fs::File;
use std::io::{ BufReader, BufRead };

pub fn run() {
    let file: File = File::open("inputs/day2.txt").expect("File opening error");
    let input: String = BufReader::new(file).lines().next().unwrap().unwrap();

    println!("Part 1: {}", part1(parse_in(&input)));
    println!("Part 2: {}", part2(parse_in(&input)));
}

fn part1(ranges: Vec<(u128,u128)>) -> u128 {
    let mut total: u128 = 0;
    for range in ranges {
        total += build_ids(range.0, range.1).iter().sum::<u128>()
    }
    total
}

fn part2(ranges: Vec<(u128, u128)>) -> u128 {
    let mut total: u128 = 0;
    for range in ranges {
        total += build_ids_2(range.0, range.1).iter().sum::<u128>()
    }
    total
}

fn parse_in(text: &str) -> Vec<(u128, u128)> {
    text.split(',').map(|x| x.split('-').map(|y| y.parse::<u128>().unwrap()).collect_tuple().unwrap()).collect_vec()
}

fn build_ids(start: u128, end: u128) -> Vec<u128> {
    let mut ids: Vec<u128> = vec![];
    let x: u128 = start.ilog10() as u128;
    for y in (x..(end.ilog10() as u128 + 1)).filter(|&x| x % 2 != 0) { // odd powers of 10 = equal #s of digits 
        let z = y / 2;
        let mut values: Vec<u128> = (10_u128.pow(z as u32)..(10_u128.pow(z as u32 +1))).map(|x| x * 10_u128.pow(z as u32+1) + x).filter(|&x| x >= start && x <= end).collect();
        ids.append(&mut values);
    } 
    ids
}

fn build_ids_2(start: u128, end: u128) -> Vec<u128> {
    let mut ids: Vec<u128> = vec![];
    let x: u128 = start.ilog10() as u128;
    for y in x..(end.ilog10() as u128 + 1) {
        for z in (0..y).filter(|a| (y + 1) % (a + 1) == 0) { // iterate thru smaller powers of 10 that divide cleanly
            let mut values: Vec<u128> = (10_u128.pow(z as u32)..10_u128.pow(z as u32 + 1)).map(|x| {
                let mut finished = x;
                let mut copies = (y + 1) / (z + 1) - 1;
                while copies > 0 {
                    finished = finished * 10_u128.pow(z as u32 + 1);
                    finished += x;
                    copies -= 1;
                }
                finished
            }).filter(|&x| x >= start && x <= end).collect();
            ids.append(&mut values);
        }
    }
    ids.sort(); ids.dedup();
    ids
}

#[cfg(test)]
mod tests {

    use super::*;

    const EX1: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(parse_in(EX1)), 1227775554)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(parse_in(EX1)), 4174379265)
    }

}