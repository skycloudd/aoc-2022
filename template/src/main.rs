use clap::Parser;
use rustyline::Editor;
use std::fs::read_to_string;

#[derive(Parser)]
#[command(version)]
struct Cli {
    input: String,
}

fn main() {
    let args = Cli::parse();

    let mut rl = Editor::<()>::new().unwrap();

    let ask = rl.readline("run part 1? (y/n): ").unwrap();

    let input = read_to_string(&args.input).unwrap();

    if ask == "y" {
        let part_1 = part_1(&input);
        println!("Part 1: {}", part_1);
    }

    let ask = rl.readline("run part 2? (y/n): ").unwrap();

    if ask == "y" {
        let part_2 = part_2(&input);
        println!("Part 2: {}", part_2);
    }
}

fn part_1(input: &str) -> String {
    String::new()
}

fn part_2(input: &str) -> String {
    String::new()
}
