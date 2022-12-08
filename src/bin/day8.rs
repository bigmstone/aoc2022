fn parse_file(file: &str) -> Vec<Vec<u32>> {
    let mut grid = vec![];
    for line in file.rsplit('\n').rev() {
        let mut row: Vec<u32> = vec![];
        for character in line.chars() {
            row.push(character.to_string().parse().unwrap());
        }
        if !row.is_empty() {
            grid.push(row);
        }
    }

    grid
}

fn visible_trees(grid: Vec<Vec<u32>>) -> u32 {
    let mut visible = vec![];
    for (index_i, i) in grid.iter().enumerate() {
        let mut row = vec![];
        for (index_j, j) in i.iter().enumerate() {
            if index_i > 0 && index_j > 0 && index_i < grid.len() - 1 && index_j < grid[0].len() - 1
            {
                let mut visible = [true, true, true, true];

                // Left
                for y in 0..index_j {
                    if grid[index_i][y] >= *j {
                        visible[0] = false;
                    }
                }
                // Right
                for y in index_j + 1..grid[0].len() {
                    if grid[index_i][y] >= *j {
                        visible[1] = false;
                    }
                }
                // Top
                // for x in 0..index_i {
                for row in grid.iter().take(index_i) {
                    if row[index_j] >= *j {
                        visible[2] = false;
                    }
                }
                // Down
                for row in grid.iter().skip(index_i + 1) {
                    if row[index_j] >= *j {
                        visible[3] = false;
                    }
                }
                row.push(visible[0] || visible[1] || visible[2] || visible[3]);
            } else {
                row.push(true);
            }
        }
        visible.push(row);
    }

    let mut counter = 0;
    for i in visible {
        for j in i {
            if j {
                counter += 1;
            }
        }
    }

    counter
}

fn max_trees(grid: Vec<Vec<u32>>) -> u32 {
    let mut all_counts = vec![];
    for (index_i, i) in grid.iter().enumerate() {
        for (index_j, j) in i.iter().enumerate() {
            let mut count = [0, 0, 0, 0];

            // Left
            for y in (0..index_j).rev() {
                count[0] += 1;
                if grid[index_i][y] >= *j {
                    break;
                }
            }
            // Right
            for y in index_j + 1..grid[0].len() {
                count[1] += 1;
                if grid[index_i][y] >= *j {
                    break;
                }
            }
            // Top
            for x in (0..index_i).rev() {
                count[2] += 1;
                if grid[x][index_j] >= *j {
                    break;
                }
            }
            // Down

            for row in grid.iter().skip(index_i + 1) {
                count[3] += 1;
                if row[index_j] >= *j {
                    break;
                }
            }

            all_counts.push(count[0] * count[1] * count[2] * count[3]);
        }
    }

    println!("{:#?}", all_counts);

    *all_counts.iter().max().unwrap() as u32
}

fn part_one() -> u32 {
    let file = include_str!("../resources/day8/data.txt");
    let grid = parse_file(file);

    visible_trees(grid)
}

fn part_two() -> u32 {
    let file = include_str!("../resources/day8/data.txt");
    let grid = parse_file(file);

    max_trees(grid)
}

fn main() {
    println!("Part 1: {}", part_one());
    println!("Part 2: {}", part_two());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let contents = r"1201210102
2011101210
1020020330
0220210020";

        let grid = parse_file(contents);
        let expected = 10 + 10 + 2 + 2 + 9;
        assert_eq!(visible_trees(grid), expected);

        let contents = r"12021
18811
13991
10011";

        let grid = parse_file(contents);
        assert_eq!(max_trees(grid), 4);
    }
}
