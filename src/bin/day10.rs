use regex::Regex;

const CLOCK_PER_ADDX: usize = 2;
const CLOCK_PER_NOOP: usize = 1;

fn parse(file: &str) -> Vec<(String, i32)> {
    let mut inst: Vec<(String, i32)> = vec![];
    let addx_re = Regex::new(r"(addx)\s+([0-9-]+)").unwrap();
    let noop_re = Regex::new(r"(noop\s*)").unwrap();
    for line in file.rsplit('\n').rev() {
        for cap in noop_re.captures_iter(line) {
            inst.push((cap[1].to_string(), 0));
        }
        for cap in addx_re.captures_iter(line) {
            inst.push((cap[1].to_string(), cap[2].parse().unwrap()));
        }
    }

    inst
}

fn add_strength(insts: &[(String, i32)]) -> i32 {
    let clocks = vec![20, 60, 100, 140, 180, 220];

    let mut x = 1;
    let mut clock = 1;
    let mut sum: i32 = 0;
    for (inst, v) in insts {
        match inst.as_str() {
            "noop" => {
                if clocks.contains(&clock) {
                    sum += clock as i32 * x;
                }
                clock += CLOCK_PER_NOOP;
            }
            "addx" => {
                for _ in 0..CLOCK_PER_ADDX {
                    if clocks.contains(&clock) {
                        sum += clock as i32 * x;
                    }
                    clock += 1;
                }
                x += v
            }
            _ => panic!("Unknown instruction"),
        }
    }

    sum
}

fn part_one() -> i32 {
    let file = include_str!("../resources/day10/data.txt");
    let insts = parse(file);
    add_strength(&insts)
}

fn clock_draw(clock: usize, x: i32) {
    let clock = clock - 1;
    let cmod = clock as i32 % 40;
    if cmod == 0 {
        println!();
    }
    if cmod == x || cmod == x - 1 || cmod == x + 1 {
        print!("#");
    } else {
        print!(".");
    }
}

fn draw(insts: &[(String, i32)]) {
    let mut x: i32 = 1;
    let mut clock = 1;
    for (inst, v) in insts {
        match inst.as_str() {
            "noop" => {
                clock_draw(clock, x);
                clock += CLOCK_PER_NOOP;
            }
            "addx" => {
                for _ in 0..CLOCK_PER_ADDX {
                    clock_draw(clock, x);
                    clock += 1;
                }
                x += v
            }
            _ => panic!("Unknown instruction"),
        }
    }
    println!();
}

fn part_two() {
    let file = include_str!("../resources/day10/data.txt");
    let insts = parse(file);
    draw(&insts);
}

fn main() {
    println!("Part 1: {}", part_one());
    println!("Part 2:");
    part_two();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let data = r"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

        let insts = parse(data);
        assert_eq!(insts.len(), 146);
        assert_eq!(add_strength(&insts), 13140);
    }
}
