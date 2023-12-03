use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
};

const LIMITS: [(&str, u8); 3] = [("blue", 14), ("green", 13), ("red", 12)];

fn main() {
    let file = File::open("../input").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();
    buf_reader.read_to_string(&mut input).unwrap();

    println!("First part");
    let part1 = part1(input.to_string());
    println!("{}", part1);

    println!("Second part");
    let part2 = part2(input.to_string());
    println!("{}", part2);
}

fn part1(input: String) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let mut content = line.split(":");
        let game_part = content.next().unwrap();
        let game_id: u32 = game_part.split(" ").last().unwrap().parse().unwrap();
        if within_limit(content.next().unwrap().to_string()) {
            sum += game_id;
        }
    }
    sum
}

fn within_limit(cubes_part: String) -> bool {
    let sets = cubes_part.split(";");
    for set in sets {
        let cubes = set.split(",");
        for cube in cubes {
            let cube = cube.strip_prefix(" ").unwrap();
            let mut spl = cube.split(" ");
            let amount: u8 = spl.next().unwrap().parse().unwrap();
            let color = spl.next().unwrap();
            let (_, limit) = LIMITS.iter().find(|el| color == el.0).unwrap();
            if amount > *limit {
                return false;
            }
        }
    }
    true
}

fn part2(input: String) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let mut content = line.split(":");
        let _ = content.next();
        sum += game_power(content.next().unwrap().to_string());
    }
    sum
}

fn game_power(cubes_part: String) -> u32 {
    let mut amounts: HashMap<&str, u32> = HashMap::new();
    amounts.insert("red", 0);
    amounts.insert("green", 0);
    amounts.insert("blue", 0);

    let sets = cubes_part.split(";");
    for set in sets {
        let cubes = set.split(",");
        for cube in cubes {
            let cube = cube.strip_prefix(" ").unwrap();
            let mut spl = cube.split(" ");
            let amount: u32 = spl.next().unwrap().parse().unwrap();
            let color = spl.next().unwrap();
            let res_amount = amounts.get(color).unwrap();
            if amount > *res_amount {
                amounts.insert(color, amount);
            }
        }
    }
    amounts
        .into_iter()
        .map(|e| e.1)
        .reduce(|n1, n2| n1 * n2)
        .unwrap()
}
