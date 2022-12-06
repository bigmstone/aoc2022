fn solver(length: usize) -> Option<u32> {
    let file = include_str!("../resources/day6/data.txt");

    for i in length..file.len() {
        let mut buffer = vec![];

        for j in 0..length {
            buffer.push(&file[i - j..i - j + 1]);
        }

        buffer.sort();
        buffer.dedup();

        if buffer.len() == length {
            return Some(i as u32 + 1);
        }
    }
    None
}

fn main() {
    println!("Part 1: {}", solver(4).unwrap());
    println!("Part 2: {}", solver(14).unwrap());
}
