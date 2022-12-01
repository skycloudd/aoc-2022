use clap::Parser;
use rustyline::Editor;
use std::fs::read_to_string;

#[derive(Parser)]
#[command(version)]
struct Cli {
    input1: String,
    input2: String,
}

fn main() {
    let args = Cli::parse();

    let mut rl = Editor::<()>::new().unwrap();

    let ask = rl.readline("run part 1? (y/n): ").unwrap();

    if ask == "y" {
        let part_1 = part_1(&args.input1);
        println!("Part 1: {}", part_1);
    }

    let ask = rl.readline("run part 2? (y/n): ").unwrap();

    if ask == "y" {
        let part_2 = part_2(&args.input2);
        println!("Part 2: {}", part_2);
    }
}

fn part_1(input_path: &str) -> i32 {
    let _input = read_to_string(input_path).unwrap();

    0
}

fn part_2(input_path: &str) -> i32 {
    let _input = read_to_string(input_path).unwrap();

    0
}
