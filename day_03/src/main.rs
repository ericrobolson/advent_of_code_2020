use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines(file_name: &'static str) -> Vec<String> {
    let path = Path::new(file_name);

    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut result = vec![];
    for line in lines {
        let line = line.unwrap();
        result.push(line);
    }

    result
}

fn main() {
    println!("Part 1: {:?} trees.", count_trees(3, 1));

    let mut num_trees = 1;
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    for (x_slope, y_slope) in slopes {
        let n = count_trees(x_slope, y_slope);
        num_trees *= n;
    }

    println!("Part 2: {:?} trees.", num_trees);
}

const EMPTY: char = '.';
const TREE: char = '#';

fn count_trees(slope_x: usize, slope_y: usize) -> i32 {
    let mut num_trees = 0;

    let lines = read_lines("input.txt");

    let mut terrain = vec![];

    for line in lines {
        let mut row = vec![];

        for c in line.chars() {
            if c == TREE || c == EMPTY {
                let is_tree = c == TREE;
                row.push(is_tree);
            }
        }

        terrain.push(row);
    }

    let mut x = 0;
    let mut y = 0;
    loop {
        if y >= terrain.len() {
            break;
        }

        if terrain[y][x] {
            num_trees += 1;
        }

        x += slope_x;
        x = x % terrain[y].len();

        y += slope_y;
    }

    num_trees
}
