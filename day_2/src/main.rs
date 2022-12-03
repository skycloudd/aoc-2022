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
    let mut points = 0;

    for game in input.lines() {
        let (opponent, player) = (
            Choice::from_char(game.chars().nth(0).unwrap()),
            Choice::from_char(game.chars().nth(2).unwrap()),
        );

        let result = Result::calculate(&opponent, &player);

        points += player.points() + result.points();
    }

    points.to_string()
}

fn part_2(input: &str) -> String {
    let mut points = 0;

    for game in input.lines() {
        let (opponent, desired_result) = (
            Choice::from_char(game.chars().nth(0).unwrap()),
            Result::from_char(game.chars().nth(2).unwrap()),
        );

        let player = match desired_result {
            Result::Win => opponent.get_winning_against(),
            Result::Draw => opponent,
            Result::Lose => opponent.get_losing_against(),
        };

        points += player.points() + Result::calculate(&opponent, &player).points();
    }

    points.to_string()
}

#[derive(Clone, Copy)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn from_char(c: char) -> Choice {
        match c {
            'A' => Choice::Rock,
            'B' => Choice::Paper,
            'C' => Choice::Scissors,
            'X' => Choice::Rock,
            'Y' => Choice::Paper,
            'Z' => Choice::Scissors,
            _ => panic!(),
        }
    }

    fn points(&self) -> u32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    fn get_winning_against(&self) -> Choice {
        match self {
            Choice::Rock => Choice::Paper,
            Choice::Paper => Choice::Scissors,
            Choice::Scissors => Choice::Rock,
        }
    }

    fn get_losing_against(&self) -> Choice {
        match self {
            Choice::Rock => Choice::Scissors,
            Choice::Paper => Choice::Rock,
            Choice::Scissors => Choice::Paper,
        }
    }
}

enum Result {
    Win,
    Lose,
    Draw,
}

impl Result {
    fn from_char(c: char) -> Self {
        match c {
            'X' => Self::Lose,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => panic!(),
        }
    }

    fn calculate(opponent: &Choice, player: &Choice) -> Result {
        match (opponent, player) {
            (Choice::Rock, Choice::Rock) => Result::Draw,
            (Choice::Rock, Choice::Paper) => Result::Win,
            (Choice::Rock, Choice::Scissors) => Result::Lose,
            (Choice::Paper, Choice::Rock) => Result::Lose,
            (Choice::Paper, Choice::Paper) => Result::Draw,
            (Choice::Paper, Choice::Scissors) => Result::Win,
            (Choice::Scissors, Choice::Rock) => Result::Win,
            (Choice::Scissors, Choice::Paper) => Result::Lose,
            (Choice::Scissors, Choice::Scissors) => Result::Draw,
        }
    }

    fn points(&self) -> u32 {
        match self {
            Result::Win => 6,
            Result::Draw => 3,
            Result::Lose => 0,
        }
    }
}
