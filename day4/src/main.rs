use std::fs::File;
use std::io::BufRead;

static DX: [i32; 8] = [0, 1, 1, 1, 0, -1, -1, -1];
static DY: [i32; 8] = [1, 1, 0, -1, -1, -1, 0, 1];
const EXPECTED_STRING: &str = "XMAS";
// const EXPECTED_STRING_PART2: &str = "MAS";

fn xmas(grid: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    let mut answer: i32 = 0;

    if grid[x][y] == 'M' && x + 2 < grid.len() && y + 2 < grid[0].len() {
        if grid[x][y + 2] == 'M'
            && grid[x + 1][y + 1] == 'A'
            && grid[x + 2][y + 2] == 'S'
            && grid[x + 2][y] == 'S'
        {
            answer += 1;
        }

        if grid[x + 2][y] == 'M'
            && grid[x + 1][y + 1] == 'A'
            && grid[x][y + 2] == 'S'
            && grid[x + 2][y + 2] == 'S'
        {
            answer += 1;
        }
    }

    if grid[x][y] == 'S' && x + 2 < grid.len() && y + 2 < grid[0].len() {
        if grid[x][y + 2] == 'S'
            && grid[x + 1][y + 1] == 'A'
            && grid[x + 2][y + 2] == 'M'
            && grid[x + 2][y] == 'M'
        {
            answer += 1;
        }

        if grid[x + 2][y] == 'S'
            && grid[x + 1][y + 1] == 'A'
            && grid[x][y + 2] == 'M'
            && grid[x + 2][y + 2] == 'M'
        {
            answer += 1;
        }
    }

    answer
}

fn part2(grid: &Vec<Vec<char>>) -> i32 {
    let mut answer = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'M' || grid[i][j] == 'S' {
                answer += xmas(grid, i, j);
            }
        }
    }
    answer
}

fn is_xmas(grid: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    let mut answer: i32 = 0;
    for i in 0..DX.len() {
        let mut curr_x = x as i32;
        let mut curr_y = y as i32;
        let mut travel_string: String = String::new();
        let mut j = 0;
        // println!("x: {} y: {} j: {}", curr_x, curr_y, j);
        while curr_x >= 0
            && (curr_x as usize) < grid.len()
            && curr_y >= 0
            && (curr_y as usize) < grid[0].len()
            && j < EXPECTED_STRING.len()
        {
            travel_string.push(grid[curr_x as usize][curr_y as usize]);
            curr_x += DX[i];
            curr_y += DY[i];
            j += 1;
        }
        // println!("Travel String: {}", travel_string);
        if travel_string == EXPECTED_STRING {
            answer += 1;
        }
    }
    answer
}

fn part1(grid: &Vec<Vec<char>>) -> i32 {
    let mut answer = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] != 'X' {
                continue;
            }
            answer += is_xmas(grid, i, j);
        }
    }
    answer
}

fn main() {
    // let file = File::open("assets/sample.txt").expect("File not Found!");
    let file = File::open("assets/input.txt").expect("File not Found!");
    let reader = std::io::BufReader::new(file);

    let grid: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.expect("Invalid line").chars().collect())
        .collect();
    // for line in reader.lines() {
    //     let line = line.expect("Error in line");
    //     let mut row: Vec<char> = Vec::new();
    //     for c in line.chars() {
    //         row.push(c);
    //     }
    //     grid.push(row);
    // }
    let answer = part1(&grid);
    println!("{}", answer);

    let answer2 = part2(&grid);
    println!("{}", answer2);
}
