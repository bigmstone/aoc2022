struct Range {
    start: u32,
    end: u32,
}

fn parse_file(file_contents: &str) -> Vec<Vec<Range>> {
    let divided = file_contents.rsplit('\n').rev();
    let mut results = vec![];
    for item in divided {
        if item.is_empty() {
            continue;
        }
        let groups = item.rsplit(',').rev();
        let mut result = vec![];
        for group in groups {
            let range: Vec<&str> = group.rsplit('-').rev().collect();
            let range = Range {
                start: range[0].parse().unwrap(),
                end: range[1].parse().unwrap(),
            };
            result.push(range);
        }
        results.push(result);
    }
    results
}

fn contained_range(range1: &Range, range2: &Range) -> bool {
    (range1.start >= range2.start && range1.end <= range2.end)
        || (range2.start >= range1.start && range2.end <= range1.end)
}

fn overlap_range(range1: &Range, range2: &Range) -> bool {
    (range1.start <= range2.end && range1.end >= range2.end)
        || (range2.start <= range1.end && range2.end >= range1.end)
}

fn part_one() -> u32 {
    let file = include_str!("../resources/day4/data.txt");
    let ranges = parse_file(file);

    let mut count = 0;
    for range in ranges {
        if contained_range(&range[0], &range[1]) {
            count += 1;
        }
    }

    count
}

fn part_two() -> u32 {
    let file = include_str!("../resources/day4/data.txt");
    let ranges = parse_file(file);

    let mut count = 0;
    for range in ranges {
        if overlap_range(&range[0], &range[1]) {
            count += 1;
        }
    }

    count
}

fn main() {
    println!("Part 1: {}", part_one());
    println!("Part 2: {}", part_two());
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn basic_contains() {
        let range1 = Range { start: 0, end: 5 };
        let range2 = Range { start: 3, end: 4 };
        assert!(contained_range(&range1, &range2));
    }

    #[test]
    fn basic_doesnt_contain() {
        let range1 = Range { start: 0, end: 5 };
        let range2 = Range { start: 5, end: 6 };
        assert!(!contained_range(&range1, &range2));
    }

    #[test]
    fn adv_contains() {
        let range1 = Range { start: 0, end: 5 };
        let range2 = Range { start: 4, end: 4 };
        assert!(contained_range(&range1, &range2));

        let range1 = Range { start: 0, end: 5 };
        let range2 = Range { start: 0, end: 1 };
        assert!(contained_range(&range1, &range2));

        let range1 = Range { start: 0, end: 5 };
        let range2 = Range { start: 0, end: 0 };
        assert!(contained_range(&range1, &range2));

        let range1 = Range { start: 1, end: 3 };
        let range2 = Range { start: 0, end: 5 };
        assert!(contained_range(&range1, &range2));

        let range1 = Range { start: 5, end: 5 };
        let range2 = Range { start: 0, end: 5 };
        assert!(contained_range(&range1, &range2));

        let range1 = Range { start: 0, end: 0 };
        let range2 = Range { start: 0, end: 5 };
        assert!(contained_range(&range1, &range2));

        let range1 = Range { start: 0, end: 5 };
        let range2 = Range { start: 0, end: 0 };
        assert!(contained_range(&range1, &range2));

        let range1 = Range { start: 0, end: 5 };
        let range2 = Range { start: 5, end: 5 };
        assert!(contained_range(&range1, &range2));
    }

    #[test]
    fn adv_doesnt_contain() {
        let range1 = Range { start: 0, end: 5 };
        let range2 = Range { start: 1, end: 6 };
        assert!(!contained_range(&range1, &range2));

        let range1 = Range { start: 1, end: 6 };
        let range2 = Range { start: 0, end: 5 };
        assert!(!contained_range(&range1, &range2));
    }

    #[test]
    fn overlap() {
        let range1 = Range { start: 1, end: 6 };
        let range2 = Range { start: 0, end: 3 };
        assert!(overlap_range(&range1, &range2));

        let range1 = Range { start: 0, end: 3 };
        let range2 = Range { start: 1, end: 6 };
        assert!(overlap_range(&range1, &range2));
    }

    #[test]
    fn not_overlap() {
        //     range1.start <= range2.end || range2.start <= range1.end
        let range1 = Range { start: 4, end: 6 };
        let range2 = Range { start: 0, end: 3 };
        assert!(!overlap_range(&range1, &range2));

        let range1 = Range { start: 0, end: 3 };
        let range2 = Range { start: 4, end: 6 };
        assert!(!overlap_range(&range1, &range2));
    }
}
