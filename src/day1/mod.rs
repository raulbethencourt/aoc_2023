use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
    u32,
};

pub fn solve_problem() -> u32 {
    let mut result: u32 = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        let re: Regex = Regex::new(r"[0-9]+").unwrap();

        lines.flatten().for_each(|line| {
            let captures: Vec<&str> = re.find_iter(&line).map(|m| m.as_str()).collect();

            let captures_lens = captures.len();

            let first_capture_numbers: Vec<char> = captures.first().unwrap().chars().collect();
            let first_number = first_capture_numbers[0].to_string();

            let second_capture_numbers: Vec<char> =
                captures.get(captures_lens - 1).unwrap().chars().collect();
            let len_second_number = second_capture_numbers.len();
            let second_number = second_capture_numbers[len_second_number - 1].to_string();

            let number = format!("{first_number}{second_number}");

            result += number.parse::<u32>().unwrap();
        });
    }
    result
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
