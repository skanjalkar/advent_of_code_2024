use std::fs::File;
use std::io::BufRead;

fn part2(line: &String, do_donot: &mut bool) -> i32 {
    let size = line.len();
    let chars: Vec<char> = line.chars().collect();
    let mut answer: i32 = 0;
    let mut i: usize = 0;
    while i < size {
        // only valid word is mul(num1,num2) everything else is skip
        if i + 4 < size && &line[i..i + 4] == "mul(" && *do_donot {
            let mut num1: i32 = 0;
            let mut num2: i32 = 0;
            let mut j = i + 4;
            if j < size && !chars[j].is_digit(10) {
                // update i to j
                i = j;
                continue;
            }
            let mut valid1: bool = false;
            while j < size && chars[j].is_digit(10) {
                if let Some(d) = chars[j].to_digit(10) {
                    num1 = num1 * 10 + d as i32;
                    valid1 = true;
                } else {
                    valid1 = false;
                    break;
                }
                j += 1;
            }
            if !valid1 {
                i = j;
                continue;
            }
            if j < size && chars[j] == ',' {
                j += 1;
            } else {
                i = j;
                continue;
            }

            let mut valid2: bool = false;
            while j < size && chars[j].is_digit(10) {
                if let Some(d) = chars[j].to_digit(10) {
                    num2 = num2 * 10 + d as i32;
                    valid2 = true;
                } else {
                    valid2 = false;
                    break;
                }
                j += 1;
            }
            if !valid2 {
                i = j;
                continue;
            }
            if j < size && chars[j] == ')' {
                println!("Found mul({}, {}) at position {}", num1, num2, i);
                answer += num1 * num2;
                i = j + 1;
            } else {
                i = j;
            }
        } else if i + 4 < size && &line[i..i + 4] == "do()" {
            println!("Found do() at position {}", i);
            *do_donot = true;
            i += 4;
        } else if i + 7 < size && &line[i..i + 7] == "don't()" {
            println!("Found don't() at position {}", i);
            *do_donot = false;
            i += 7;
        } else {
            // println!("Character is {}", chars[i]);
            i += 1;
        }
    }
    return answer;
}

fn part1(line: &String) -> i32 {
    let size = line.len();
    let chars: Vec<char> = line.chars().collect();
    let mut answer: i32 = 0;
    let mut i: usize = 0;
    while i < size {
        // only valid word is mul(num1,num2) everything else is skip
        if i + 4 < size && &line[i..i + 4] == "mul(" {
            let mut num1: i32 = 0;
            let mut num2: i32 = 0;
            let mut j = i + 4;
            if j < size && !chars[j].is_digit(10) {
                // update i to j
                i = j;
                continue;
            }
            let mut valid1: bool = false;
            while j < size && chars[j].is_digit(10) {
                if let Some(d) = chars[j].to_digit(10) {
                    num1 = num1 * 10 + d as i32;
                    valid1 = true;
                } else {
                    valid1 = false;
                    break;
                }
                j += 1;
            }
            if !valid1 {
                i = j;
                continue;
            }
            if j < size && chars[j] == ',' {
                j += 1;
            } else {
                i = j;
                continue;
            }

            let mut valid2: bool = false;
            while j < size && chars[j].is_digit(10) {
                if let Some(d) = chars[j].to_digit(10) {
                    num2 = num2 * 10 + d as i32;
                    valid2 = true;
                } else {
                    valid2 = false;
                    break;
                }
                j += 1;
            }
            if !valid2 {
                i = j;
                continue;
            }
            if j < size && chars[j] == ')' {
                answer += num1 * num2;
                i = j + 1;
            } else {
                i = j;
            }
        } else {
            i += 1;
        }
    }
    return answer;
}

fn main() {
    let sample = File::open("assets/input.txt").unwrap();
    let reader = std::io::BufReader::new(sample);
    // very few liens but big lines
    // xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))

    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    println!("Line are");
    let mut answer: i32 = 0;
    for line in &lines {
        // println!("{}", line);
        answer += part1(line);
    }
    println!("Answer: {}", answer);
    let mut answer2: i32 = 0;
    let mut do_donot: bool = true;
    for line in &lines {
        // println!("{}", line);
        answer2 += part2(line, &mut do_donot);
        println!("Running total: {}", answer2);
    }
    println!("Answer2: {}", answer2);
}
