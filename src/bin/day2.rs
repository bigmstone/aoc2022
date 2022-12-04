#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Selection {
    Rock = 1,
    Paper = 2,
    Sissors = 3,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Outcome {
    Won = 6,
    Draw = 3,
    Lost = 0,
}

fn determine_outcome(oponent_selection: Selection, selection: Selection) -> Outcome {
    if oponent_selection == selection {
        return Outcome::Draw;
    }

    if oponent_selection == Selection::Rock && selection != Selection::Paper {
        return Outcome::Lost;
    }

    if oponent_selection == Selection::Paper && selection != Selection::Sissors {
        return Outcome::Lost;
    }

    if oponent_selection == Selection::Sissors && selection != Selection::Rock {
        return Outcome::Lost;
    }

    Outcome::Won
}

fn string_to_selection_part_1(file_contents: &str) -> Vec<[Selection; 2]> {
    let divided = file_contents.rsplit('\n').rev();
    let mut results: Vec<[Selection; 2]> = vec![];
    for item in divided {
        let mut result: [Selection; 2] = [Selection::Rock, Selection::Rock];
        if item.len() < 2 {
            continue;
        }
        for (index, item) in item.split_whitespace().enumerate() {
            let selection = match item {
                "A" => Selection::Rock,
                "B" => Selection::Paper,
                "C" => Selection::Sissors,
                "X" => Selection::Rock,
                "Y" => Selection::Paper,
                "Z" => Selection::Sissors,
                _ => {
                    panic!("No Valid Selection")
                }
            };
            result[index] = selection;
        }
        results.push(result);
    }

    results
}

fn lose_to(selection: Selection) -> Selection {
    if selection == Selection::Rock {
        Selection::Sissors
    } else if selection == Selection::Paper {
        Selection::Rock
    } else {
        Selection::Paper
    }
}

fn win_against(selection: Selection) -> Selection {
    if selection == Selection::Rock {
        Selection::Paper
    } else if selection == Selection::Paper {
        Selection::Sissors
    } else {
        Selection::Rock
    }
}

fn string_to_selection_part_2(file_contents: &str) -> Vec<[Selection; 2]> {
    let divided = file_contents.rsplit('\n').rev();
    let mut results: Vec<[Selection; 2]> = vec![];
    for item in divided {
        let mut result: [Selection; 2] = [Selection::Rock, Selection::Rock];
        if item.len() < 2 {
            continue;
        }
        for (index, item) in item.split_whitespace().enumerate() {
            let selection = match item {
                "A" => Selection::Rock,
                "B" => Selection::Paper,
                "C" => Selection::Sissors,
                "X" => lose_to(result[0]),
                "Y" => result[0],
                "Z" => win_against(result[0]),
                _ => {
                    panic!("No Valid Selection")
                }
            };
            result[index] = selection;
        }
        results.push(result);
    }

    results
}

fn sum_outcomes_from_selections(selections: Vec<[Selection; 2]>) -> u32 {
    let mut results = vec![];
    for selection in selections {
        results.push((determine_outcome(selection[0], selection[1]), selection[1]));
    }

    results
        .iter()
        .map(|result| result.0 as u32 + result.1 as u32)
        .sum::<u32>()
}

fn part_one() -> u32 {
    let file = include_str!("../resources/day2/strategy_guide.txt");
    let selections = string_to_selection_part_1(file);
    sum_outcomes_from_selections(selections)
}

fn part_two() -> u32 {
    let file = include_str!("../resources/day2/strategy_guide.txt");
    let selections = string_to_selection_part_2(file);
    sum_outcomes_from_selections(selections)
}

fn main() {
    println!("Part 1: {}", part_one());
    println!("Part 2: {}", part_two());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_win() {
        assert_eq!(
            determine_outcome(Selection::Rock, Selection::Paper),
            Outcome::Won
        );
    }

    #[test]
    fn test_loss() {
        assert_eq!(
            determine_outcome(Selection::Rock, Selection::Sissors),
            Outcome::Lost
        );
    }

    #[test]
    fn test_draw() {
        assert_eq!(
            determine_outcome(Selection::Rock, Selection::Rock),
            Outcome::Draw
        );
    }

    #[test]
    fn test_logic() {
        assert_eq!(
            determine_outcome(Selection::Rock, Selection::Rock),
            Outcome::Draw
        );
        assert_eq!(
            determine_outcome(Selection::Rock, Selection::Paper),
            Outcome::Won
        );
        assert_eq!(
            determine_outcome(Selection::Rock, Selection::Sissors),
            Outcome::Lost
        );
        assert_eq!(
            determine_outcome(Selection::Paper, Selection::Paper),
            Outcome::Draw
        );
        assert_eq!(
            determine_outcome(Selection::Paper, Selection::Sissors),
            Outcome::Won
        );
        assert_eq!(
            determine_outcome(Selection::Paper, Selection::Rock),
            Outcome::Lost
        );
        assert_eq!(
            determine_outcome(Selection::Sissors, Selection::Sissors),
            Outcome::Draw
        );
        assert_eq!(
            determine_outcome(Selection::Sissors, Selection::Rock),
            Outcome::Won
        );
        assert_eq!(
            determine_outcome(Selection::Sissors, Selection::Paper),
            Outcome::Lost
        );
    }

    #[test]
    fn test_sum() {
        let selections = vec![
            [Selection::Rock, Selection::Paper],
            [Selection::Rock, Selection::Paper],
            [Selection::Rock, Selection::Paper],
        ];
        assert_eq!(sum_outcomes_from_selections(selections), 24);

        let selections = vec![
            [Selection::Rock, Selection::Paper],
            [Selection::Rock, Selection::Rock],
            [Selection::Rock, Selection::Paper],
        ];
        assert_eq!(sum_outcomes_from_selections(selections), 20);

        let selections = vec![
            [Selection::Rock, Selection::Paper], // Win (6) + Paper (2)    = 8
            [Selection::Rock, Selection::Rock],  // Draw (3) + Rock (1)    = 4
            [Selection::Rock, Selection::Sissors], // Loss (0) + Sissors (3) = 3
        ];
        assert_eq!(sum_outcomes_from_selections(selections), 15);

        let selections = vec![
            [Selection::Rock, Selection::Paper], // Win (6) + Paper (2)    = 8
            [Selection::Paper, Selection::Rock], // Loss (3) + Rock (1)    = 1
            [Selection::Sissors, Selection::Sissors], // Draw (0) + Sissors (3) = 6
        ];
        assert_eq!(sum_outcomes_from_selections(selections), 15);

        let selections = vec![
            [Selection::Rock, Selection::Rock], //       Draw (3)  + Rock (1)       = 4
            [Selection::Rock, Selection::Paper], //      Win  (6)  + Paper (2)      = 8
            [Selection::Rock, Selection::Sissors], //    Lost (0)  + Sissors (3)    = 3
            [Selection::Paper, Selection::Rock], //      Lost (0)  + Rock (1)       = 1
            [Selection::Paper, Selection::Paper], //     Draw (3)  + Paper (2)      = 5
            [Selection::Paper, Selection::Sissors], //   Win  (6)  + Sissors (3)    = 9
            [Selection::Sissors, Selection::Rock], //    Win  (6)  + Rock (1)       = 7
            [Selection::Sissors, Selection::Paper], //   List (0)  + Paper (2)      = 2
            [Selection::Sissors, Selection::Sissors], // Draw (3)  + Sissors (3)    = 6
        ];
        assert_eq!(sum_outcomes_from_selections(selections), 45);

        let selections = vec![
            [Selection::Rock, Selection::Sissors], //  Lost (0)  + Sissors (3) = 3
            [Selection::Sissors, Selection::Rock], //  Win  (6)  + Rock    (1) = 7
            [Selection::Rock, Selection::Sissors], //  Lost (0)  + Sissors (3) = 3
            [Selection::Rock, Selection::Sissors], //  Lost (0)  + Sissors (3) = 3
            [Selection::Sissors, Selection::Paper], // Lost (0)  + Paper   (2) = 2
            [Selection::Sissors, Selection::Paper], // Lost (0)  + Paper   (2) = 2
            [Selection::Rock, Selection::Sissors], //  Lost (0)  + Sissors (3) = 3
            [Selection::Rock, Selection::Paper],   //  Win  (6)  + Paper   (2) = 8
            [Selection::Sissors, Selection::Paper], // Lost (0)  + Paper   (2) = 2
        ];
        assert_eq!(sum_outcomes_from_selections(selections), 33);
    }

    #[test]
    fn test_string_parse() {
        let string = "A Z\nC X\nA Z\nA Z\nC Y\nC Y\nA Z\nA Y\nC Y";
        let results = string_to_selection_part_1(string);

        assert_eq!(
            results,
            vec![
                [Selection::Rock, Selection::Sissors],
                [Selection::Sissors, Selection::Rock],
                [Selection::Rock, Selection::Sissors],
                [Selection::Rock, Selection::Sissors],
                [Selection::Sissors, Selection::Paper],
                [Selection::Sissors, Selection::Paper],
                [Selection::Rock, Selection::Sissors],
                [Selection::Rock, Selection::Paper],
                [Selection::Sissors, Selection::Paper],
            ]
        );

        assert_eq!(
            sum_outcomes_from_selections(results),
            3 + 7 + 3 + 3 + 2 + 2 + 3 + 8 + 2
        );
    }
}
