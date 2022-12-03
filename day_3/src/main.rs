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

    let input = read_to_string(&args.input).unwrap();

    let ask = rl.readline("run part 1? (y/n): ").unwrap();

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
    let mut total = 0;

    for line in input.lines() {
        let length = line.len();
        let first_half = &line[..length / 2];
        let second_half = &line[length / 2..];

        let intersection = first_half
            .chars()
            .filter(|c| second_half.contains(*c))
            .collect::<String>()
            .chars()
            .nth(0)
            .unwrap();

        let priority = get_priority(intersection) as u32;

        total += priority;
    }

    total.to_string()
}

fn part_2(input: &str) -> String {
    let lines = input.lines().collect::<Vec<&str>>();

    let mut total = 0;

    for group in lines.chunks(3) {
        let intersection = group[0]
            .chars()
            .filter(|c| group[1].contains(*c) && group[2].contains(*c))
            .collect::<String>()
            .chars()
            .nth(0)
            .unwrap();

        let priority = get_priority(intersection) as u32;

        total += priority;
    }

    total.to_string()
}

fn get_priority(c: char) -> u8 {
    match c {
        'a'..='z' => c as u8 - 'a' as u8 + 1,
        'A'..='Z' => c as u8 - 'A' as u8 + 27,
        _ => panic!(),
    }
}
