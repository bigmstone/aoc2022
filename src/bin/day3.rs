use std::collections::HashSet;

#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug, Ord, PartialOrd)]
#[allow(non_camel_case_types)]
enum Priority {
    a = 1,
    b = 2,
    c = 3,
    d = 4,
    e = 5,
    f = 6,
    g = 7,
    h = 8,
    i = 9,
    j = 10,
    k = 11,
    l = 12,
    m = 13,
    n = 14,
    o = 15,
    p = 16,
    q = 17,
    r = 18,
    s = 19,
    t = 20,
    u = 21,
    v = 22,
    w = 23,
    x = 24,
    y = 25,
    z = 26,
    A = 27,
    B = 28,
    C = 29,
    D = 30,
    E = 31,
    F = 32,
    G = 33,
    H = 34,
    I = 35,
    J = 36,
    K = 37,
    L = 38,
    M = 39,
    N = 40,
    O = 41,
    P = 42,
    Q = 43,
    R = 44,
    S = 45,
    T = 46,
    U = 47,
    V = 48,
    W = 49,
    X = 50,
    Y = 51,
    Z = 52,
}

impl From<char> for Priority {
    fn from(ch: char) -> Self {
        match ch {
            'a' => Priority::a,
            'b' => Priority::b,
            'c' => Priority::c,
            'd' => Priority::d,
            'e' => Priority::e,
            'f' => Priority::f,
            'g' => Priority::g,
            'h' => Priority::h,
            'i' => Priority::i,
            'j' => Priority::j,
            'k' => Priority::k,
            'l' => Priority::l,
            'm' => Priority::m,
            'n' => Priority::n,
            'o' => Priority::o,
            'p' => Priority::p,
            'q' => Priority::q,
            'r' => Priority::r,
            's' => Priority::s,
            't' => Priority::t,
            'u' => Priority::u,
            'v' => Priority::v,
            'w' => Priority::w,
            'x' => Priority::x,
            'y' => Priority::y,
            'z' => Priority::z,
            'A' => Priority::A,
            'B' => Priority::B,
            'C' => Priority::C,
            'D' => Priority::D,
            'E' => Priority::E,
            'F' => Priority::F,
            'G' => Priority::G,
            'H' => Priority::H,
            'I' => Priority::I,
            'J' => Priority::J,
            'K' => Priority::K,
            'L' => Priority::L,
            'M' => Priority::M,
            'N' => Priority::N,
            'O' => Priority::O,
            'P' => Priority::P,
            'Q' => Priority::Q,
            'R' => Priority::R,
            'S' => Priority::S,
            'T' => Priority::T,
            'U' => Priority::U,
            'V' => Priority::V,
            'W' => Priority::W,
            'X' => Priority::X,
            'Y' => Priority::Y,
            'Z' => Priority::Z,
            _ => panic!("Unsupported char"),
        }
    }
}

fn find_dup(pack: Vec<Priority>) -> Option<Priority> {
    let (pack1, pack2) = pack.split_at(pack.len() / 2);
    let mut uniq = HashSet::new();

    for item in pack1 {
        uniq.insert(item);
    }

    let mut pack2 = pack2.to_owned();
    pack2.sort();
    pack2.dedup();
    for item in pack2.iter() {
        if !uniq.insert(item) {
            return Some(*item);
        }
    }

    None
}

fn parse_file(file_contents: &str) -> Vec<Vec<Priority>> {
    let divided = file_contents.rsplit('\n').rev();
    let mut results = vec![];
    for item in divided {
        if item.is_empty() {
            continue;
        }
        let mut result = vec![];
        for item in item.chars() {
            result.push(Priority::from(item))
        }
        results.push(result);
    }
    results
}

fn part_one() -> u32 {
    let file = include_str!("../resources/day3/data.txt");
    let packs = parse_file(file);
    let mut dups = vec![];
    for pack in packs {
        if let Some(dup) = find_dup(pack) {
            dups.push(dup as u32);
        }
    }
    dups.iter().sum::<u32>()
}

fn find_unique(packs: Vec<Vec<Priority>>) -> u32 {
    let mut result = vec![];
    for group in packs.chunks(3) {
        let mut combined = vec![];
        let mut uniq = HashSet::new();
        for member in group {
            let mut member_pack = member.to_owned();
            member_pack.sort();
            member_pack.dedup();
            for item in member_pack {
                combined.push(item);
                uniq.insert(item);
            }
        }
        combined.sort();
        for key in uniq {
            let mut count = 0;
            for item in combined.iter() {
                if key == *item {
                    count += 1;
                }
                if count == 3 {
                    result.push(*item as u32);
                    break;
                }
            }
        }
    }

    result.iter().sum::<u32>()
}

fn part_two() -> u32 {
    let file = include_str!("../resources/day3/data.txt");
    let packs = parse_file(file);
    find_unique(packs)
}

fn main() {
    println!("Part 1: {}", part_one());
    println!("Part 2: {}", part_two());
}
