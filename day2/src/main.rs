use std::fs::File;
use std::io::BufRead;

fn part2(v2: &Vec<i32>) -> bool {
    if part1(v2) {
        return true;
    }

    for i in 0..v2.len() {
        let mut test_vec: Vec<i32> = v2.to_vec();
        test_vec.remove(i);
        if part1(&test_vec) {
            return true;
        }
    }
    return false;
}

fn part1(v1: &Vec<i32>) -> bool {
    let size: i32 = v1.len() as i32;
    assert!(size > 1);
    let mut diff: i32 = v1[0] - v1[1];
    if diff < 1 || diff > 3 {
        return false;
    }

    for i in 1..size - 1 {
        diff = v1[i as usize] - v1[(i + 1) as usize];
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    return true;
}

fn main() {
    // let sample = File::open("assets/sample.txt").unwrap();
    let input = File::open("assets/input.txt").unwrap();
    // let reader = std::io::BufReader::new(sample);
    let reader = std::io::BufReader::new(input);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let mut answer = 0;
    let mut answer2 = 0;
    for line in &lines {
        // strip the whitespaces and convert it to vec<i32>
        let mut v1: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        if part1(&v1) {
            answer += 1;
        } else {
            // reverse v1
            v1.reverse();
            if part1(&v1) {
                answer += 1;
            }
        }
    }
    println!("Answer: {}", answer);
    for line in &lines {
        // strip the whitespaces and convert it to vec<i32>
        let mut v1: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        if part2(&v1) {
            answer2 += 1;
        } else {
            // reverse v1
            v1.reverse();
            if part2(&v1) {
                answer2 += 1;
            }
        }
    }
    println!("Answer2: {}", answer2);
}
