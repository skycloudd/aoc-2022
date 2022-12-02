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
    let mut highest = 0;
    let mut current_elf = 0;

    for line in input.lines() {
        if line.is_empty() {
            if current_elf > highest {
                highest = current_elf;
            }
            current_elf = 0;

            continue;
        } else {
            let count = line.parse::<i32>().unwrap();

            current_elf += count;
        }
    }

    highest.to_string()
}

fn part_2(input: &str) -> String {
    let mut elves = vec![];

    let mut current_elf = 0;

    for line in input.lines() {
        if line.is_empty() {
            elves.push(current_elf);

            current_elf = 0;

            continue;
        } else {
            let count = line.parse::<i32>().unwrap();

            current_elf += count;
        }
    }

    elves.sort_by(|a, b| b.cmp(a));

    (elves[0] + elves[1] + elves[2]).to_string()
}
