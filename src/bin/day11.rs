use regex::Regex;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum OpComponent {
    #[default]
    Old,
    Int(u32),
}

impl From<&str> for OpComponent {
    fn from(comp: &str) -> Self {
        if comp == "old" {
            Self::Old
        } else {
            Self::Int(comp.parse().unwrap())
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Op {
    #[default]
    Add,
    Mult,
}

impl From<&str> for Op {
    fn from(op: &str) -> Self {
        match op {
            "*" => Self::Mult,
            "+" => Self::Add,
            _ => panic!("Unknown Operand"),
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Operation {
    op: Op,
    left: OpComponent,
    right: OpComponent,
}

#[derive(Debug, Default, Clone, Copy)]
struct Test {
    div: u32,
    t: usize,
    f: usize,
}

#[derive(Debug, Default, Clone)]
struct Monkey {
    inspects: u32,
    items: Vec<u32>,
    operation: Operation,
    test: Test,
}

fn parse_items(items: &str) -> Vec<u32> {
    let re = Regex::new(r"([0-9]+)+[,\s]*").unwrap();
    let mut result = vec![];
    let caps = re.captures_iter(items);

    for cap in caps {
        let mut cap = cap.iter();
        cap.next();
        for item in cap {
            result.push(item.unwrap().as_str().parse().unwrap());
        }
    }

    result
}

fn parse_operation(operation: &str) -> Operation {
    let re = Regex::new(r"(.*)\s([*+])\s(.*)").unwrap();

    let mut result = Operation::default();

    for cap in re.captures_iter(operation) {
        result.op = Op::from(&cap[2]);
        result.left = OpComponent::from(&cap[1]);
        result.right = OpComponent::from(&cap[3]);
    }

    result
}

fn parse_test(test: &str, tru: &str, fls: &str) -> Test {
    let re = Regex::new(r"divisible\sby\s(.*)").unwrap();
    let t_re = Regex::new(r"throw\sto\smonkey\s(.*)").unwrap();
    let f_re = Regex::new(r"throw\sto\smonkey\s(.*)").unwrap();

    let mut result = Test::default();

    for cap in re.captures_iter(test) {
        result.div = cap[1].parse().unwrap();
    }

    for cap in t_re.captures_iter(tru) {
        result.t = cap[1].parse().unwrap();
    }

    for cap in f_re.captures_iter(fls) {
        result.f = cap[1].parse().unwrap();
    }

    result
}

fn parse(data: &str) -> Vec<Monkey> {
    let data = data.rsplit("\n\n");
    let re = Regex::new(
        r"\s{2}Starting items: (.*)
\s{2}Operation:\snew\s=\s(.*)
\s{2}Test:\s(.*)
\s{4}If\strue:\s(.*)
\s{4}If\sfalse:\s(.*)",
    )
    .unwrap();

    let mut monkeys = vec![];
    for block in data {
        for cap in re.captures_iter(block) {
            let items = parse_items(&cap[1]);
            let operation = parse_operation(&cap[2]);
            monkeys.push(Monkey {
                items,
                operation,
                test: parse_test(&cap[3], &cap[4], &cap[5]),
                ..Default::default()
            });
        }
    }

    monkeys.iter().rev().cloned().collect()
}

fn simulate(rounds: usize, monkeys: &mut [Monkey]) {
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let mut moves = vec![];
            for item in monkey.items.drain(..) {
                let left: u64 = match monkey.operation.left {
                    OpComponent::Old => item,
                    OpComponent::Int(i) => i,
                } as u64;
                let right: u64 = match monkey.operation.right {
                    OpComponent::Old => item,
                    OpComponent::Int(i) => i,
                } as u64;
                let item = match monkey.operation.op {
                    Op::Add => left + right,
                    Op::Mult => left * right,
                } / 3;
                monkey.inspects += 1;
                if item % monkey.test.div as u64 == 0 {
                    moves.push((item, monkey.test.t));
                } else {
                    moves.push((item, monkey.test.f));
                }
            }
            for mv in moves {
                monkeys[mv.1 as usize].items.push(mv.0 as u32);
            }
        }
    }
}

fn simulate_big(rounds: usize, monkeys: &mut [Monkey]) {
    let lcd: u32 = monkeys
        .iter()
        .scan(1, |s, c| {
            *s *= c.test.div;
            Some(*s as u32)
        })
        .last()
        .unwrap();

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let mut moves = vec![];
            for item in monkey.items.drain(..) {
                let left: u64 = match monkey.operation.left {
                    OpComponent::Old => item,
                    OpComponent::Int(i) => i,
                } as u64;
                let right: u64 = match monkey.operation.right {
                    OpComponent::Old => item,
                    OpComponent::Int(i) => i,
                } as u64;
                let item = match monkey.operation.op {
                    Op::Add => left + right,
                    Op::Mult => left * right,
                };
                monkey.inspects += 1;
                if item % monkey.test.div as u64 == 0 {
                    moves.push((item % lcd as u64, monkey.test.t));
                } else {
                    moves.push((item % lcd as u64, monkey.test.f));
                }
            }
            for mv in moves {
                monkeys[mv.1 as usize].items.push(mv.0 as u32);
            }
        }
    }
}

fn part_one(data: &str) -> u32 {
    let mut monkeys = parse(data);
    simulate(20, &mut monkeys);

    let mut results = vec![];

    for monkey in monkeys {
        results.push(monkey.inspects);
    }

    results.sort();

    results[results.len() - 1] * results[results.len() - 2]
}

fn part_two(data: &str) -> u64 {
    let mut monkeys = parse(data);
    simulate_big(10000, &mut monkeys);

    let mut results = vec![];

    for monkey in monkeys {
        results.push(monkey.inspects as u64);
    }

    results.sort();

    results[results.len() - 1] * results[results.len() - 2]
}

fn main() {
    let file = include_str!("../resources/day11/data.txt");
    println!("Part One: {}", part_one(file));
    println!("Part Two: {}", part_two(file));
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = r"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_parse() {
        let monkeys = parse(DATA);

        // Check Items
        assert_eq!(monkeys[0].items[0], 79);
        assert_eq!(monkeys[0].items[1], 98);
        assert_eq!(monkeys[1].items[0], 54);
        assert_eq!(monkeys[1].items[1], 65);
        assert_eq!(monkeys[1].items[2], 75);
        assert_eq!(monkeys[1].items[3], 74);
        assert_eq!(monkeys[2].items[0], 79);
        assert_eq!(monkeys[2].items[1], 60);
        assert_eq!(monkeys[2].items[2], 97);
        assert_eq!(monkeys[3].items[0], 74);

        // Check Operation
        assert_eq!(monkeys[0].operation.op, Op::Mult);
        assert_eq!(monkeys[1].operation.op, Op::Add);
        assert_eq!(monkeys[2].operation.op, Op::Mult);
        assert_eq!(monkeys[3].operation.op, Op::Add);
    }

    #[test]
    fn test_simulate() {
        let mut monkeys = parse(DATA);
        simulate(20, &mut monkeys);

        let expected = [101, 95, 7, 105];

        for (i, expect) in expected.iter().enumerate() {
            assert_eq!(expect, &monkeys[i].inspects);
        }
    }

    #[test]
    fn test_result_part_one() {
        assert_eq!(part_one(DATA), 10605);
    }

    #[test]
    fn test_simulate_big() {
        let mut monkeys = parse(DATA);
        simulate_big(1, &mut monkeys);

        let expected = [2, 4, 3, 6];

        for (i, expect) in expected.iter().enumerate() {
            assert_eq!(expect, &monkeys[i].inspects);
        }

        let mut monkeys = parse(DATA);
        simulate_big(20, &mut monkeys);

        let expected = [99, 97, 8, 103];

        for (i, expect) in expected.iter().enumerate() {
            assert_eq!(expect, &monkeys[i].inspects);
        }
    }

    #[test]
    fn test_result_part_two() {
        assert_eq!(part_two(DATA), 2713310158);
    }
}
