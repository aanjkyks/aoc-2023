use std::{
    fs::File,
    io::{BufReader, Read},
};

fn main() {
    let file = File::open("input").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();
    buf_reader.read_to_string(&mut input).unwrap();

    println!("First step");
    let step_one: u32 = input.lines().map(|line| first_step(line.to_string())).sum();
    println!("{}", step_one);

    println!("Second step");
    let step_two: u32 = input
        .lines()
        .map(|line| second_step(line.to_string()))
        .sum();
    println!("{}", step_two);
}

fn first_step(input: String) -> u32 {
    let filter = &mut input.chars().filter(|c| c.is_digit(10));

    let mut first = '+';
    let mut last = '+';
    for c in filter {
        if first == '+' {
            first = c;
        }
        last = c;
    }

    let fst_res = first.to_digit(10).or_else(|| Some(0)).unwrap();
    let snd_res = last.to_digit(10).or_else(|| Some(0)).unwrap();

    let result = fst_res * 10 + snd_res;
    result
}

fn second_step(input: String) -> u32 {
    const DIGITS: [(&str, u8); 10] = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("zero", 0),
    ];

    let mut parsed = input.clone();
    for (word, digit) in DIGITS {
        parsed = parsed.replace(word, &format!("{}{}{}", word, digit, word).to_string())
    }
    parsed = parsed.chars().filter(|c| c.is_digit(10)).collect();

    first_step(parsed.clone())
}
