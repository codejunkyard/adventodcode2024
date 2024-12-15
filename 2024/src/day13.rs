use advent_lib::fetch_input;
use dotenv::dotenv;
use regex::Regex;
use std::env;
use std::error::Error;

pub fn solve() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    let url = "https://adventofcode.com/2024/day/13/input";
    let session_token = env::var("SESSION_TOKEN").expect("SESSION_TOKEN must be set");

    let input = fetch_input(url, &session_token)?;
    //let input = "Button A: X+94, Y+34\nButton B: X+22, Y+67\nPrize: X=8400, Y=5400\n\nButton A: X+26, Y+66\nButton B: X+67, Y+21\nPrize: X=12748, Y=12176\n\nButton A: X+17, Y+86\nButton B: X+84, Y+37\nPrize: X=7870, Y=6450\n\nButton A: X+69, Y+23\nButton B: X+27, Y+71\nPrize: X=18641, Y=10279";

    let systems_of_equations: Vec<LinearSystem2x2> = transform(&input);

    let count_part_1 = get_part_1(&systems_of_equations);
    println!("Part 1: Total count: {}", count_part_1);

    let count_part_2 = get_part_2(&systems_of_equations);
    println!("Part 2: Total count: {}", count_part_2);

    Ok(())
}

fn get_part_1(systems_of_equations: &Vec<LinearSystem2x2>) -> f64 {
    let mut sum: f64 = 0.0;

    for linear_system in systems_of_equations {
        println!("Coefficients: {:?}", linear_system.coefficients);
        println!("Constants: {:?}", linear_system.constants);

        let det_a = determinant_2x2(linear_system.coefficients);

        if det_a == 0.0 {
            println!("The system has no unique solution.");
            break;
        }

        // Replace first column with B to compute det(A_A)
        let a_a = [
            [linear_system.constants[0], linear_system.coefficients[0][1]],
            [linear_system.constants[1], linear_system.coefficients[1][1]],
        ];
        let det_a_a = determinant_2x2(a_a);

        // Replace second column with B to compute det(A_B)
        let a_b = [
            [linear_system.coefficients[0][0], linear_system.constants[0]],
            [linear_system.coefficients[1][0], linear_system.constants[1]],
        ];
        let det_a_b = determinant_2x2(a_b);

        // Compute solutions
        let solution_a = det_a_a / det_a;
        let solution_b = det_a_b / det_a;

        if solution_a.fract() != 0.0 || solution_b.fract() != 0.0 {
            println!("No solution for this type of problem");
        } else {
            println!(
                "Solution: A = {:.2}, B = {:.2}",
                solution_a * 3.0,
                solution_b
            );
            println!("Sum: {}", solution_a * 3.0 + solution_b);

            sum += solution_a * 3.0 + solution_b;
        }
    }

    sum
}

fn get_part_2(systems_of_equations: &Vec<LinearSystem2x2>) -> usize {
    let mut sum = 0;

    sum
}

struct LinearSystem2x2 {
    coefficients: [[f64; 2]; 2],
    constants: [f64; 2],
}

impl LinearSystem2x2 {
    /// Creates a new LinearSystem2x2
    fn new(coefficients: [[f64; 2]; 2], constants: [f64; 2]) -> Self {
        Self {
            coefficients,
            constants,
        }
    }
}

fn transform(input: &str) -> Vec<LinearSystem2x2> {
    let mut systems_of_equations: Vec<LinearSystem2x2> = Vec::new();
    let systems_input: Vec<_> = input.trim().split("\n\n").collect();
    //println!("{}", systems_input.len());

    // Define the regex patterns
    let button_a_regex = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
    let button_b_regex = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    for linear_system in systems_input {
        let mut x_a: Option<f64> = None;
        let mut y_a: Option<f64> = None;
        let mut x_b: Option<f64> = None;
        let mut y_b: Option<f64> = None;
        let mut p_a: Option<f64> = None;
        let mut p_b: Option<f64> = None;

        // Parse Button A
        if let Some(caps) = button_a_regex.captures(linear_system) {
            x_a = caps[1].parse().ok();
            y_a = caps[2].parse().ok();
        }

        // Parse Button B
        if let Some(caps) = button_b_regex.captures(linear_system) {
            x_b = caps[1].parse().ok();
            y_b = caps[2].parse().ok();
        }

        // Parse Prize
        if let Some(caps) = prize_regex.captures(linear_system) {
            p_a = caps[1].parse().ok();
            p_b = caps[2].parse().ok();
        }

        // Ensure all values are defined
        if let (Some(x_a), Some(y_a), Some(x_b), Some(y_b), Some(p_a), Some(p_b)) =
            (x_a, y_a, x_b, y_b, p_a, p_b)
        {
            systems_of_equations.push(LinearSystem2x2::new(
                [[x_a, x_b], [y_a, y_b]],
                [p_a + 10000000000000.0, p_b + 10000000000000.0],
            ));
        } else {
            eprintln!("Failed to parse input: {}", input);
        }
    }

    systems_of_equations
}

fn determinant_2x2(matrix: [[f64; 2]; 2]) -> f64 {
    (matrix[0][0] * matrix[1][1]) - (matrix[0][1] * matrix[1][0])
}
