use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
// sort

fn part1(v1: &mut Vec<i32>, v2: &mut Vec<i32>) {
    let mut distance: i32 = 0;
    // sort v1 and v2

    v1.sort();
    v2.sort();
    for i in 0..v1.len() {
        let diff = v1[i] - v2[i];
        distance += diff.abs();
    }
    println!("Distance: {}", distance);
}

fn part2(v1: Vec<i32>, v2: Vec<i32>) {
    let mut similarty_score: i32 = 0;
    let mut map: HashMap<i32, i32> = HashMap::new();
    for i in 0..v2.len() {
        let number: i32 = v2[i];
        // insert in map if not there otherwise incremenet

        if map.contains_key(&number) {
            let count = map.get(&number).unwrap();
            map.insert(number, count + 1);
        } else {
            map.insert(number, 1);
        }
    }
    for i in 0..v1.len() {
        let number: i32 = v1[i];
        if map.contains_key(&number) {
            let count = map.get(&number).unwrap();
            if *count > 0 {
                similarty_score += number * count;
            }
        }
    }
    println!("Similarity Score: {}", similarty_score);
}

fn main() {
    // read from assets/sample.txt
    // let input = File::open("assets/sample.txt").unwrap();
    let input = File::open("assets/input.txt").unwrap();
    let reader = std::io::BufReader::new(input);
    // data in the following format
    // 1 2
    // 2 3
    // 689689 814929038

    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    println!("Lines are: ");
    for line in &lines {
        println!("{}", line);
    }
    // split into two vecotors
    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();
    for line in &lines {
        let mut parts = line.split_whitespace();
        let first: i32 = parts.next().unwrap().parse().unwrap();
        let second: i32 = parts.next().unwrap().parse().unwrap();
        v1.push(first);
        v2.push(second);
        println!("Part 1 is: {} and Part 2 is {}", first, second);
    }
    part1(&mut v1, &mut v2);
    part2(v1, v2);
}
