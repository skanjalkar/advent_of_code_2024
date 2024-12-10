use std::fs::File;
use std::io::{BufRead, BufReader};
// hashmap
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn dfs(graph: &HashMap<i32, Vec<i32>>, paths: &Vec<i32>, i: usize) -> bool {
    if i == (paths.len() - 1) {
        return true;
    }
    if let Some(neighbors) = graph.get(&paths[i]) {
        if neighbors.contains(&paths[i + 1]) {
            return dfs(graph, paths, i + 1);
        }
    }
    false
}

fn topological_sort(graph: &HashMap<i32, Vec<i32>>, path: &Vec<i32>) -> Vec<i32> {
    let path_set: HashSet<i32> = path.iter().cloned().collect();
    let mut result = Vec::new();
    let mut in_degree: HashMap<i32, i32> = HashMap::new();
    let mut queue: VecDeque<i32> = VecDeque::new();
    let mut already_inserted: HashSet<i32> = HashSet::new();

    // Initialize in-degree for nodes in this path only
    for &node in path {
        in_degree.insert(node, 0);
    }

    // Calculate in-degree only for edges between nodes in this path
    for &node in path {
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if path_set.contains(&neighbor) {
                    *in_degree.get_mut(&neighbor).expect("Expected i32") += 1;
                }
            }
        }
    }

    for (&node, &degree) in &in_degree {
        if degree == 0 {
            queue.push_back(node);
        }
    }

    while let Some(node) = queue.pop_front() {
        if already_inserted.contains(&node) {
            continue;
        }
        result.push(node);
        already_inserted.insert(node);

        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if path_set.contains(&neighbor) {
                    *in_degree.get_mut(&neighbor).expect("Expected i32") -= 1;
                    if in_degree[&neighbor] == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }
    result
}

fn part2(graph: HashMap<i32, Vec<i32>>, paths: &Vec<Vec<i32>>) {
    // make all permutations of path
    let mut answer: i32 = 0;
    for path in paths {
        if !dfs(&graph, path, 0) {
            let sorted: Vec<i32> = topological_sort(&graph, path);
            println!("{:?}", sorted);
            answer += sorted[sorted.len() / 2];
        }
    }
    // return answer;
    println!("{}", answer);
}

fn part1(graph: &HashMap<i32, Vec<i32>>, paths: &Vec<Vec<i32>>) {
    let mut answer: i32 = 0;
    for path in paths {
        if dfs(graph, path, 0) {
            answer += path[path.len() / 2];
        }
    }
    println!("{}", answer);
}

fn main() {
    // let file = File::open("assets/sample.txt").expect("File not found");
    let file = File::open("assets/input.txt").expect("File not found");
    let reader = BufReader::new(file);
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut paths: Vec<Vec<i32>> = Vec::new();
    let mut flag: bool = true;
    for line in reader.lines() {
        let l = line.expect("Invalid line");
        if flag {
            if l.is_empty() {
                flag = false;
                continue;
            }
            // split | and add to graph
            let numbers: (i32, i32) = l
                .split("|")
                .map(|x| x.parse::<i32>().expect("Invalid number"))
                .collect_tuple()
                .expect("Invalid pair");
            graph.entry(numbers.0).or_insert(Vec::new()).push(numbers.1);
        } else {
            // iter
            paths.push(
                l.split(",")
                    .map(|x| x.parse::<i32>().expect("Invalid number"))
                    .collect(),
            )
        }
    }
    println!("{:?}", graph);
    println!("{:?}", paths);
    part1(&graph, &paths);
    part2(graph, &paths);
}
