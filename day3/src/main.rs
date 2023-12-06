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
.664.598..";

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
    let step_one: u32 = first_step(INPUT.to_string());
    println!("{}", step_one);

    println!("Second step");
    let step_two: u32 = 0;
    println!("{}", step_two);
}

fn first_step(input: String) -> u32 {
    let matrix = &mut Matrix::new(&input).unwrap();
    let matrix_copy = matrix.clone(); // lmao rust get fukked ( T_T rip memory )
    let mut sum = 0;

    println!("matrix = {:?}", matrix);

    for i in 0..matrix.val.len() {
        let line = matrix.val.get_mut(i).unwrap();
        for j in 0..line.len() {
            println!(
                "accessing ({i},{j}), size = ({},{})",
                matrix_copy.val.len(),
                line.len()
            );
            let cell = line.get_mut(j).unwrap();
            println!("cell = {:?}", cell);
            if cell.visited {
                println!("visited");
                continue;
            }
            cell.visited = true;
            if cell.c.is_digit(10) {
                println!("found digit {}", cell.c);
                let mut num = vec![cell.c];
                let mut k = j + 1;
                loop {
                    let mut cell = line.get_mut(k); // wtf is as_ref and as_mut
                    if !cell.as_ref().is_some_and(|cl| cl.c.is_digit(10)) {
                        break;
                    }
                    println!("in loop {:?}", cell);
                    cell.as_mut().unwrap().visited = true;
                    num.push(cell.unwrap().c);
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
                    .map(|(row, col)| matrix_copy.get(row, col))
                    .any(|cell| cell.is_some());
                if relevant {
                    let number_str: String = num.iter().collect();
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

#[derive(Clone, Debug)]
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

    pub fn get(&self, row: usize, col: usize) -> Option<Cell> {
        let cell = self.val.get(row)?.get(col)?;
        if cell.c == EMPTY_SYMBOL {
            return None;
        }
        Some(*cell)
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut Cell> {
        let cell: &mut Cell = self.val.get_mut(row)?.get_mut(col)?;
        if cell.c == EMPTY_SYMBOL {
            return None;
        }
        Some(cell)
    }
}
