use advent_of_code_2024::solutions;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let solution_name = args.get(1).expect("Provide specific solution name");

    for (name, solution) in solutions() {
        if name == solution_name {
            let input_file = format!("resource/input/{}.txt", name.split('_').next().unwrap());
            let input = fs::read_to_string(input_file).expect("Input file does not exist in 'resource/input/'");

            println!("{}", solution(input.trim().lines().collect()));
        }
    }
}
