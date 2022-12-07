use std::collections::HashMap;

use regex::Regex;

#[derive(Debug)]
struct File {
    size: u32,
}

#[derive(Default, Debug)]
struct Directory {
    directories: HashMap<String, Directory>,
    files: Vec<File>,
}

fn parse_file(file: &str) -> Directory {
    let directories = HashMap::new();
    let files = vec![];
    let mut root_dir = Directory { directories, files };
    let mut current_dir = &mut root_dir;
    let mut dir_stack = vec![];

    let re_cd = Regex::new(r"\$\scd\s(.*)").unwrap();
    let re_dir = Regex::new(r"dir\s(.*)").unwrap();
    let re_file = Regex::new(r"([0-9]+)\s(.*)").unwrap();
    let lines = file.rsplit('\n').rev();
    for line in lines {
        for cap in re_cd.captures_iter(line) {
            match &cap[1] {
                "/" => {}
                ".." => {
                    dir_stack.pop().unwrap();
                    current_dir = &mut root_dir;
                    for dir in &dir_stack {
                        current_dir = current_dir.directories.get_mut(dir).unwrap();
                    }
                }
                _ => {
                    dir_stack.push(String::from(&cap[1]));
                    current_dir = current_dir.directories.get_mut(&cap[1]).unwrap();
                }
            }
        }
        for cap in re_dir.captures_iter(line) {
            current_dir
                .directories
                .insert(String::from(&cap[1]), Directory::default());
        }
        for cap in re_file.captures_iter(line) {
            current_dir.files.push(File {
                size: cap[1].parse().unwrap(),
            });
        }
    }

    root_dir
}

fn sum_files_lt(dir: &Directory, dirs: &mut Vec<u32>) -> u32 {
    let mut sum = 0;
    for file in &dir.files {
        sum += file.size;
    }

    for dir in dir.directories.values() {
        sum += sum_files_lt(dir, dirs);
    }

    if sum <= 100000 {
        dirs.push(sum);
    }

    sum
}

fn sum_used(dir: &Directory) -> u32 {
    let mut sum = 0;
    for file in &dir.files {
        sum += file.size;
    }

    for dir in dir.directories.values() {
        sum += sum_used(dir);
    }

    sum
}

fn collect_options(dir: &Directory, options: &mut Vec<u32>, space_needed: u32) -> u32 {
    let mut sum = 0;
    for file in &dir.files {
        sum += file.size;
    }

    for dir in dir.directories.values() {
        sum += collect_options(dir, options, space_needed);
    }

    if sum >= space_needed {
        options.push(sum);
    }

    sum
}

fn part_one() -> u32 {
    let dir = parse_file(include_str!("../resources/day7/data.txt"));
    let mut dirs_under = vec![];
    sum_files_lt(&dir, &mut dirs_under);

    dirs_under.iter().sum()
}

fn part_two() -> u32 {
    let fs_size = 70000000;
    let required_space = 30000000;
    let dir = parse_file(include_str!("../resources/day7/data.txt"));
    let used_space = sum_used(&dir);

    let remaining_space = fs_size - used_space;
    let space_needed = required_space - remaining_space;

    let mut options = vec![];
    collect_options(&dir, &mut options, space_needed);

    *options.iter().min().unwrap()
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
        let contents = r"$ cd /
$ ls
dir ctd
dir bob
80649 mwcj.pmh
212527 nbb.ztq
152170 scr.smr
17637 snqcgbs.nhv
$ cd ctd
$ ls
152170 scr.smr
17637 snqcgbs.nhv
$ cd ..
$ cd bob
1521 bobf1.lk
177 bobf2.lk
";

        let root_dir = parse_file(contents);
        assert_eq!(root_dir.files[0].size, 80649);
        assert_eq!(root_dir.files[1].size, 212527);
        assert_eq!(root_dir.files[2].size, 152170);
        assert_eq!(root_dir.files[3].size, 17637);
        assert_eq!(
            root_dir.directories.get("ctd").unwrap().files[0].size,
            152170
        );
        assert_eq!(
            root_dir.directories.get("ctd").unwrap().files[1].size,
            17637
        );
        assert_eq!(root_dir.directories.get("bob").unwrap().files[0].size, 1521);
        assert_eq!(root_dir.directories.get("bob").unwrap().files[1].size, 177);
    }
}
