use regex::Regex;

fn parse_boxes(boxes: String) -> Vec<Vec<String>> {
    let mut parsed_diagram = vec![];
    let mut boxes: Vec<&str> = boxes.rsplit('\n').rev().collect();
    boxes.pop();
    for (row, item) in boxes.iter().enumerate() {
        let item = item.chars().collect::<Vec<char>>();
        for (column, item) in item.chunks(4).enumerate() {
            if item.len() < 3 {
                continue;
            }
            let c = item[1];
            if c != ' ' {
                parsed_diagram.push((row, column, c));
            }
        }
    }

    let mut result = vec![vec![]; 9];

    for b in parsed_diagram {
        result[b.1].push(b.2.to_string());
    }

    for row in result.iter_mut() {
        row.reverse();
    }

    result
}

fn parse_moves(moves: String) -> Vec<(u32, u32, u32)> {
    let mut result = vec![];

    let re = Regex::new(r"move\s+(\d+)\s+from+\s(\d+)\s+to\s+(\d+)").unwrap();
    for cap in re.captures_iter(&moves) {
        result.push((
            cap[1].parse().unwrap(),
            cap[2].parse().unwrap(),
            cap[3].parse().unwrap(),
        ));
    }

    result
}

fn parse_file(file_contents: &str) -> (String, String) {
    let divided: Vec<&str> = file_contents.rsplit("\n\n").collect();
    let boxes = divided[1].to_string();
    let moves = divided[0].to_string();
    (boxes, moves)
}

fn part_one() -> String {
    let file = include_str!("../resources/day5/data.txt");
    let (boxes, moves) = parse_file(file);
    let mut boxes = parse_boxes(boxes);
    let moves = parse_moves(moves);

    for mv in moves {
        for _ in 0..mv.0 {
            let b = boxes[(mv.1 - 1) as usize].pop().unwrap();
            boxes[(mv.2 - 1) as usize].push(b);
        }
    }

    let mut r = String::from("");

    for col in boxes {
        if !col.is_empty() {
            r.push_str(col.last().unwrap());
        }
    }

    r
}

fn part_two() -> String {
    let file = include_str!("../resources/day5/data.txt");
    let (boxes, moves) = parse_file(file);
    let mut boxes = parse_boxes(boxes);
    let moves = parse_moves(moves);

    for mv in moves {
        if mv.0 > 1 {
            let b = {
                let src_row = &mut boxes[(mv.1 - 1) as usize];
                let b: Vec<String> = src_row
                    .drain((src_row.len() - (mv.0) as usize)..src_row.len())
                    .collect();

                b
            };
            let dst_row = &mut boxes[(mv.2 - 1) as usize];
            dst_row.extend(b);
            // boxes[(mv.2 - 1) as usize].push(b);
        } else {
            let b = boxes[(mv.1 - 1) as usize].pop().unwrap();
            boxes[(mv.2 - 1) as usize].push(b);
        }
    }

    let mut r = String::from("");

    for col in boxes {
        if !col.is_empty() {
            r.push_str(col.last().unwrap());
        }
    }

    r
}

fn main() {
    println!("Part 1: {}", part_one());
    println!("Part 2: {}", part_two());
}
