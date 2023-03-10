use std::fs;
use std::time::Instant;

use fancy_regex::Regex;
use lazy_static::lazy_static;

const PROBLEM_NAME: &str = "Matchsticks";
const PROBLEM_INPUT_FILE: &str = "./input/day08.txt";
const PROBLEM_DAY: u64 = 8;

lazy_static! {
    static ref REGEX_HEX: Regex = Regex::new(r#"\\x[0-9a-f][0-9a-f]"#).unwrap();
    static ref REGEX_QUOTE: Regex = Regex::new(r#"\\\""#).unwrap();
    static ref REGEX_SLASH: Regex = Regex::new(r#"\\\\"#).unwrap();
}

/// Processes the AOC 2015 Day 08 input file and solves both parts of the problem. Solutions are
/// printed to stdout.
pub fn main() {
    let start = Instant::now();
    // Input processing
    let input = process_input_file(PROBLEM_INPUT_FILE);
    let input_parser_timestamp = Instant::now();
    let input_parser_duration = input_parser_timestamp.duration_since(start);
    // Solve part 1
    let p1_solution = solve_part1(&input);
    let p1_timestamp = Instant::now();
    let p1_duration = p1_timestamp.duration_since(input_parser_timestamp);
    // Solve part 2
    let p2_solution = solve_part2(&input);
    let p2_timestamp = Instant::now();
    let p2_duration = p2_timestamp.duration_since(p1_timestamp);
    // Print results
    println!("==================================================");
    println!("AOC 2015 Day {} - \"{}\"", PROBLEM_DAY, PROBLEM_NAME);
    println!("[+] Part 1: {}", p1_solution);
    println!("[+] Part 2: {}", p2_solution);
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("Execution times:");
    println!("[+] Input:  {:.2?}", input_parser_duration);
    println!("[+] Part 1: {:.2?}", p1_duration);
    println!("[+] Part 2: {:.2?}", p2_duration);
    println!(
        "[*] TOTAL:  {:.2?}",
        input_parser_duration + p1_duration + p2_duration
    );
    println!("==================================================");
}

/// Processes the AOC 2015 Day 08 input file into the format required by the solver functions.
/// Returned value is a vector of strings given as lines in the input file.
fn process_input_file(filename: &str) -> Vec<String> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input
        .trim()
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
}

/// Solves AOC 2015 Day 08 Part 1 // Determines the difference between the total number of
/// characters in the "in-code" and "in-memory" representations of the input strings.
fn solve_part1(input_strings: &[String]) -> usize {
    let mut chars_code = 0;
    let mut chars_mem = 0;
    for s in input_strings {
        // Find the in-mem representation of string - '#' used as placeholder
        let mut s_mem = REGEX_SLASH.replace_all(s, "#").to_string();
        s_mem = REGEX_QUOTE.replace_all(&s_mem, "#").to_string();
        s_mem = REGEX_HEX.replace_all(&s_mem, "#").to_string();
        // Add to in-code and in-mem length totals
        chars_code += s.len();
        chars_mem += s_mem.len() - 2; // Exclude open and close double-quotes from in-mem length
    }
    chars_code - chars_mem
}

/// Solves AOC 2015 Day 08 Part 2 // Determines the difference between the total number of
/// characters in the new-encoding and in-code representations of the input strings.
fn solve_part2(input_strings: &[String]) -> usize {
    let mut chars_encoded = 0;
    let mut chars_code = 0;
    for s in input_strings {
        // Find the new encoded representation of string
        let mut new_s = s.replace('\\', "\\\\");
        new_s = new_s.replace('"', "\\\"");
        // Add to new-encoding and in-code length totals
        chars_code += s.len();
        chars_encoded += new_s.len() + 2; // Include new open and close double-quotes in encoded len
    }
    chars_encoded - chars_code
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 08 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day08_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(1371, solution);
    }

    /// Tests the Day 08 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day08_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(2117, solution);
    }
}
