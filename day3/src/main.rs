use std::{
    fs::File,
    io::{BufReader, Read},
};

const INPUT: &str = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

const INPUT_SMOL: &str = "
467.
...*
";

const EMPTY_SYMBOL: char = '.';

fn main() {
    let file = File::open("../input").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();
    buf_reader.read_to_string(&mut input).unwrap();

    println!("First step");
    let step_one: u32 = first_step(INPUT_SMOL.to_string());
    println!("{}", step_one);

    println!("Second step");
    let step_two: u32 = 0;
    println!("{}", step_two);
}

fn first_step(input: String) -> u32 {
    let mut matrix = Matrix::new(&input).unwrap();
    let mut sum = 0;
    for (i, line) in matrix.val().iter().enumerate() {
        for (j, cell) in line.iter().enumerate() {
            println!("cell = {:?}", cell);
            let visited = cell.visited;
            let char = cell.c;
            if visited {
                println!("visited");
                continue;
            }
            if char.is_digit(10) {
                let mut num = vec![char];
                let mut k = j + 1;
                while matrix.get(i, k).is_some_and(|c| c.c.is_digit(10)) {
                    num.push(matrix.get(i, k).map(|c| c.c).unwrap());
                    k += 1;
                }
                let own_rows = i..i + 1;
                let own_cols = j..k + 1;
                let mut own_cells = own_rows
                    .map(|x| std::iter::repeat(x).zip(own_cols.clone()))
                    .flatten();

                let i_low = if i == 0 { 0 } else { i - 1 };
                let j_low = if j == 0 { 0 } else { j - 1 };
                let rows = i_low..i + 2;
                let cols = j_low..k + 2;
                let relevant = rows
                    .map(|x| std::iter::repeat(x).zip(cols.clone()))
                    .flatten()
                    .filter(|cell| own_cells.all(|own_cell| *cell != own_cell))
                    .map(|(row, col)| matrix.get(row, col))
                    .any(|cell| cell.is_some());
                if relevant {
                    let number_str: String = num.iter().map(|c| *c).collect();
                    let number: u32 = number_str.parse().unwrap();
                    println!("number={number}");
                    sum += number;
                }
            }
        }
    }
    sum
}

fn second_step(input: String) -> u32 {
    0
}

#[derive(Copy, Clone, Debug)]
struct Cell {
    c: char,
    visited: bool,
}

#[derive(Clone)]
struct Matrix {
    val: Vec<Vec<Cell>>,
}

impl Matrix {
    pub fn new(input: &str) -> Option<Self> {
        let vec: Vec<Vec<Cell>> = input
            .lines()
            .into_iter()
            .map(|line| line.chars().map(|c| Cell { c, visited: false }).collect())
            .collect();

        Some(Self { val: vec })
    }

    pub fn get(&mut self, row: usize, col: usize) -> Option<Cell> {
        let cell: &mut Cell = self.val.get_mut(row)?.get_mut(col)?;
        cell.visited = true;
        if cell.c == EMPTY_SYMBOL {
            return None;
        }
        Some(*cell)
    }

    pub fn val(&self) -> Vec<Vec<Cell>> {
        self.val.clone()
    }
}
