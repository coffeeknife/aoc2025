pub fn run() {
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
}

fn parse_line(line: &str) -> Vec<bool> {
    line.chars()
        .map(|x| if x == '@' { true } else { false })
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;
}
