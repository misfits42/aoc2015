use std::collections::HashMap;
use std::fs;
use std::time::Instant;

use lazy_static::lazy_static;

use aoc_utils::cartography::Point2D;

const PROBLEM_NAME: &str = "Like a GIF For Your Yard";
const PROBLEM_INPUT_FILE: &str = "./input/day18.txt";
const PROBLEM_DAY: u64 = 18;

lazy_static! {
    static ref LIGHTS_STUCK_ON: Vec<Point2D> = vec![
        Point2D::new(0, 0),
        Point2D::new(99, 0),
        Point2D::new(0, 99),
        Point2D::new(99, 99),
    ];
}

/// Processes the AOC 2015 Day 18 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2015 Day 18 input file into the format required by the solver functions.
/// Returned value is hashmap of lightgrid locations and initial light state (true: on, false: off).
fn process_input_file(filename: &str) -> HashMap<Point2D, bool> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let mut lightgrid: HashMap<Point2D, bool> = HashMap::new();
    for (y, row) in raw_input.trim().lines().enumerate() {
        for (x, elem) in row.chars().enumerate() {
            let loc = Point2D::new(x as i64, y as i64);
            let state = match elem {
                '#' => true,
                '.' => false,
                _ => panic!("Invalid input file char! // {elem}"),
            };
            lightgrid.insert(loc, state);
        }
    }
    lightgrid
}

/// Solves AOC 2015 Day 18 Part 1 // Determines the number of lights that are left on after 100
/// steps from the initial configuration of the lightgrid.
fn solve_part1(lightgrid: &HashMap<Point2D, bool>) -> usize {
    let new_lightgrid = simulate_lightgrid(lightgrid, 100, &[]);
    new_lightgrid.values().filter(|elem| **elem).count()
}

/// Solves AOC 2015 Day 18 Part 2 // Determines the number of lights that are left on after 100
/// steps from the initial configuration of the lightgrid, with the four corner lights stuck in the
/// "on" position.
fn solve_part2(lightgrid: &HashMap<Point2D, bool>) -> usize {
    let new_lightgrid = simulate_lightgrid(lightgrid, 100, &LIGHTS_STUCK_ON);
    new_lightgrid.values().filter(|elem| **elem).count()
}

/// Simulates the given number of steps from the initial lightgrid state and returns the resulting
/// lightgrid.
fn simulate_lightgrid(
    lightgrid: &HashMap<Point2D, bool>,
    steps: u64,
    stuck_on: &[Point2D],
) -> HashMap<Point2D, bool> {
    // Initialise the lightgrid with the stuck lights set to on
    let mut old_lightgrid = lightgrid.clone();
    for stuck_loc in stuck_on {
        old_lightgrid.insert(*stuck_loc, true);
    }
    for _ in 0..steps {
        // Initialise the new lightgrid with the stuck lights set to on
        let mut new_lightgrid: HashMap<Point2D, bool> = HashMap::new();
        for stuck_loc in stuck_on {
            new_lightgrid.insert(*stuck_loc, true);
        }
        for loc in old_lightgrid.keys() {
            // Skip the stuck lights since they have already been added to the new lightgrid
            if new_lightgrid.contains_key(loc) {
                continue;
            }
            let new_state = determine_new_state(loc, &old_lightgrid);
            new_lightgrid.insert(*loc, new_state);
        }
        // Update the old lightgrid for the next step
        old_lightgrid = new_lightgrid;
    }
    old_lightgrid
}

/// Determines the new state for the light at the given location in the next step lightgrid.
fn determine_new_state(loc: &Point2D, old_lightgrid: &HashMap<Point2D, bool>) -> bool {
    // Count the number of surrounding lights that are on
    let mut count_on = 0;
    for sloc in loc.get_surrounding_points() {
        if *old_lightgrid.get(&sloc).unwrap_or(&false) {
            count_on += 1;
        }
    }
    // Determine the new state based on current state and neighbour-on count
    let new_state = match old_lightgrid.get(loc).unwrap() {
        true => count_on == 2 || count_on == 3,
        false => count_on == 3,
    };
    new_state
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 18 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day18_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(821, solution);
    }

    /// Tests the Day 18 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day18_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(886, solution);
    }
}
