use std::env;
use std::fs;

mod days;
mod solution;

fn get_solution(nr: u32) -> Box<dyn solution::Solution> {
    match nr {
        1 => return Box::new(days::day01::Day1),
        2 => return Box::new(days::day02::Day2),
        3 => return Box::new(days::day03::Day3),
        4 => return Box::new(days::day04::Day4),
        5 => return Box::new(days::day05::Day5),
        6 => return Box::new(days::day06::Day6),
        7 => return Box::new(days::day07::Day7),
        8 => return Box::new(days::day08::Day8),
        9 => return Box::new(days::day09::Day9),
        10 => return Box::new(days::day10::Day10),
        11 => return Box::new(days::day11::Day11),
        12 => return Box::new(days::day12::Day12),
        13 => return Box::new(days::day13::Day13),
        14 => return Box::new(days::day14::Day14),
        15 => return Box::new(days::day15::Day15),
        16 => return Box::new(days::day16::Day16),
        17 => return Box::new(days::day17::Day17),
        18 => return Box::new(days::day18::Day18),
        19 => return Box::new(days::day19::Day19),
        20 => return Box::new(days::day20::Day20),
        21 => return Box::new(days::day21::Day21),
        22 => return Box::new(days::day22::Day22),
        23 => return Box::new(days::day23::Day23),
        24 => return Box::new(days::day24::Day24),
        25 => return Box::new(days::day25::Day25),
        _ => panic!("Solution doesn't exist!"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let solution_nr: u32 = (&args[1]).parse().unwrap();
    let extension = if args.len() > 2 && &args[2] == "--example" { "example" } else { "main" };

    let sln = get_solution(solution_nr);
    let filename = format!("data/{:02}.{}", solution_nr, extension);
    println!("Loading from {}", filename.clone());
    let data = fs::read_to_string(&filename).expect("Unable to read file");
    println!("{}", sln.solve_p1(&data));
    println!("{}", sln.solve_p2(&data));
}
