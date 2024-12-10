use std::fs::File;
use std::io::{self, BufRead};

static DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn dfs(grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, i: i32, j: i32, dir: &mut usize) {
    if i < 0 || j < 0 || i >= grid.len() as i32 || j >= grid[0].len() as i32 {
        // println!("i: {}, j: {}", i, j);
        return;
    }
    // if i j is # change dir clockwise and call dfs
    // otherwise continue in this direction
    println!("i: {}, j: {} ", i, j);
    if grid[i as usize][j as usize] == '#' {
        let new_i = i - DIRECTIONS[*dir].0;
        let new_j = j - DIRECTIONS[*dir].1;
        *dir = (*dir + 1) % 4;
        dfs(
            grid,
            visited,
            new_i + DIRECTIONS[*dir].0,
            new_j + DIRECTIONS[*dir].1,
            dir,
        );
    } else if grid[i as usize][j as usize] == '.' || grid[i as usize][j as usize] == '^' {
        visited[i as usize][j as usize] = true;
        dfs(
            grid,
            visited,
            i + DIRECTIONS[*dir].0,
            j + DIRECTIONS[*dir].1,
            dir,
        );
    }
}

fn main() {
    // let file = File::open("./assets/sample.txt").expect("Cannot open file");
    let file = File::open("./assets/input.txt").expect("Cannot open file");
    let file = file;
    let lines = io::BufReader::new(file).lines();
    let mut start: (usize, usize) = (0, 0);
    let grid: Vec<Vec<char>> = lines
        .map(|line| line.expect("Invalid line").chars().collect())
        .collect();
    // print grid
    // for row in &grid {
    //     for col in row {
    //         print!("{}", col);
    //     }
    //     println!();
    // }

    let mut flag: bool = true;
    for (i, row) in grid.iter().enumerate() {
        for (j, &col) in row.iter().enumerate() {
            if col == '^' {
                start = (i, j);
                // println!("start: {}, {}", start.0, start.1);
                flag = false;
                break;
            }
        }
        if !flag {
            break;
        }
    }

    // resize visited with false
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    //
    dfs(&grid, &mut visited, start.0 as i32, start.1 as i32, &mut 0);
    let answer: i32 = visited
        .iter()
        .map(|x| x.iter().filter(|&y| *y).count() as i32)
        .sum();
    println!("{}", answer);
}
