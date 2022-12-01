fn part_one() -> u32 {
    let mut results: Vec<u32> = vec![];
    let file = include_str!("../resources/day1/calories.txt").rsplit("\n\n");
    for item in file {
        let mut result: Vec<u32> = vec![];
        for item in item.split_whitespace() {
            result.push(item.parse().unwrap());
        }
        results.push(result.iter().sum());
    }

    *results.iter().max().unwrap()
}

fn part_two() -> u32 {
    let mut results: Vec<u32> = vec![];
    let file = include_str!("../resources/day1/calories.txt").rsplit("\n\n");
    for item in file {
        let mut result: Vec<u32> = vec![];
        for item in item.split_whitespace() {
            result.push(item.parse().unwrap());
        }
        results.push(result.iter().sum());
    }

    results.sort();

    results[results.len() - 1] + results[results.len() - 2] + results[results.len() - 3]
}

fn main() {
    println!("Part 1: {}", part_one());
    println!("Part 2: {}", part_two());
}
