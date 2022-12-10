use std::cmp::Ordering;

use regex::Regex;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => panic!("Unknown character"),
        }
    }
}

struct Move {
    dir: Direction,
    count: u32,
}

fn build_moves(file: &str) -> Vec<Move> {
    let mut moves = vec![];
    let re = Regex::new(r"([A-z]+)\s+([0-9]+)").unwrap();
    for line in file.rsplit('\n').rev() {
        for cap in re.captures_iter(line) {
            moves.push(Move {
                dir: Direction::from(&cap[1]),
                count: cap[2].parse().unwrap(),
            });
        }
    }

    moves
}

fn attached(h: (usize, usize), t: (usize, usize)) -> bool {
    let mut attached = false;

    if (h.0 == t.0 || h.0 == t.0 - 1 || h.0 == t.0 + 1)
        && (h.1 == t.1 || h.1 == t.1 - 1 || h.1 == t.1 + 1)
    {
        attached = true;
    }

    attached
}

fn _print_move(m: &Move, k: &[(usize, usize)], size_x: usize, size_y: usize) {
    println!("Move: {:#?} by {}", m.dir, m.count);
    for _ in 0..size_x {
        print!("-");
    }
    println!();
    for i in 0..size_x {
        for j in 0..size_y {
            let mut print = " ".to_string();
            for (x, k) in k.iter().enumerate().rev() {
                if i == k.0 && j == k.1 {
                    print = format!("{}", x)
                }
            }
            print!("{}", print)
        }
        println!();
    }
}

fn calc_move(i: (usize, usize), j: (usize, usize)) -> (i32, i32) {
    let mut out = (0, 0);

    match i.0.cmp(&j.0) {
        Ordering::Greater => out.0 = 1,
        Ordering::Less => out.0 = -1,
        Ordering::Equal => {}
    }

    match i.1.cmp(&j.1) {
        Ordering::Greater => out.1 = 1,
        Ordering::Less => out.1 = -1,
        Ordering::Equal => {}
    }

    out
}

fn simulate(moves: &[Move], sims: usize) -> Vec<Vec<Vec<bool>>> {
    let size = 500;
    let half_size = size / 2;
    let mut pos = vec![(half_size, half_size); sims];

    let mut grid = vec![vec![vec![false; sims]; size]; size];
    for i in 0..sims {
        grid[pos[0].0][pos[0].1][i] = true;
    }

    for m in moves {
        for _ in 0..m.count {
            match m.dir {
                Direction::Up => {
                    pos[0].0 -= 1;
                    grid[pos[0].0][pos[0].1][0] = true;
                }
                Direction::Down => {
                    pos[0].0 += 1;
                    grid[pos[0].0][pos[0].1][0] = true;
                }
                Direction::Left => {
                    pos[0].1 -= 1;
                    grid[pos[0].0][pos[0].1][0] = true;
                }
                Direction::Right => {
                    pos[0].1 += 1;
                    grid[pos[0].0][pos[0].1][0] = true;
                }
            }
            for i in 1..sims {
                while !attached(pos[i - 1], pos[i]) {
                    let m = calc_move(pos[i - 1], pos[i]);
                    pos[i] = (
                        (pos[i].0 as i32 + m.0) as usize,
                        (pos[i].1 as i32 + m.1) as usize,
                    );
                }
                grid[pos[i].0][pos[i].1][i] = true;
            }
            // _print_move(m, &pos, size, size);
        }
    }

    grid
}

fn part_one(data: &str) -> u32 {
    let moves = build_moves(data);
    let grid = simulate(&moves, 2);

    let mut count = 0;

    for i in grid {
        for j in i {
            if j[1] {
                count += 1;
            }
        }
    }

    count
}

fn part_two(data: &str) -> u32 {
    let moves = build_moves(data);
    let grid = simulate(&moves, 10);

    let mut counts = vec![0; 10];
    for i in &grid {
        for j in i {
            for x in 0..10 {
                if j[x] {
                    counts[x] += 1;
                }
            }
        }
    }

    counts[9]
}

fn main() {
    let data = include_str!("../resources/day9/data.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_attached() {
        // 1 2 3 4 5
        // 1
        // 2 a a a
        // 3 a x a
        // 4 a a a
        // 5

        let h = (3, 3);
        let t = (2, 2);
        assert!(attached(h, t));

        let h = (3, 3);
        let t = (3, 2);
        assert!(attached(h, t));

        let h = (3, 3);
        let t = (4, 2);
        assert!(attached(h, t));

        let h = (3, 3);
        let t = (2, 3);
        assert!(attached(h, t));

        let h = (3, 3);
        let t = (3, 3);
        assert!(attached(h, t));

        let h = (3, 3);
        let t = (4, 3);
        assert!(attached(h, t));

        let h = (3, 3);
        let t = (2, 4);
        assert!(attached(h, t));

        let h = (3, 3);
        let t = (3, 4);
        assert!(attached(h, t));

        let h = (3, 3);
        let t = (4, 4);
        assert!(attached(h, t));

        let h = (3, 3);
        let t = (4, 5);
        assert!(!attached(h, t));
    }

    #[test]
    fn test_simulation_p1() {
        let moves = r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!(part_one(moves), 13);

        let moves = r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
L 2
R 5
U 1
L 4
U 1
R 3
D 4
L 4";
        assert_eq!(part_one(moves), 14);
    }

    #[test]
    fn test_simulation_p2() {
        let moves = r"R 8";
        assert_eq!(part_two(moves), 1);
        let moves = r"R 8
U 5";
        assert_eq!(part_two(moves), 2);

        let moves = r"R 5
        U 8
        L 8
        D 3
        R 17
        D 10
        L 25
        U 20";
        assert_eq!(part_two(moves), 36);
    }
}
