use std::collections::HashMap;

use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("inputs/day-7.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: String) -> Result<u32> {
    let dirs = parse_input_into_dirs(input)?;

    let dir_sizes = dir_sizes(&dirs)?;

    Ok(dir_sizes
        .into_values()
        .filter(|dir_size| *dir_size <= 100000)
        .sum())
}

fn part_2(input: String) -> Result<u32> {
    let dirs = parse_input_into_dirs(input)?;

    let dir_sizes = dir_sizes(&dirs)?;

    let space_required = 30000000 - (70000000 - dir_sizes["/"]);

    Ok(dir_sizes.into_values().fold(u32::MAX, |acc, dir_size| {
        if dir_size > space_required && dir_size < acc {
            dir_size
        } else {
            acc
        }
    }))
}

/// Represents a directory.
#[derive(Default)]
struct Dir {
    /// The total size of files directly inside this directory.
    size: u32,

    /// Subfolders (path strings) of this directory.
    child_dirs: Vec<String>,
}

fn parse_input_into_dirs(input: String) -> Result<HashMap<String, Dir>> {
    // The stack of directories in the cwd.
    let mut cwd: Vec<String> = vec![String::from("/")];

    let mut dirs: HashMap<String, Dir> = HashMap::new();
    dirs.entry(cwd.join("")).or_default();

    for line in input.lines() {
        match line {
            "$ cd /" => cwd = vec![String::from("/")],
            "$ cd .." => {
                cwd.pop();
            }
            cd if cd.starts_with("$ cd ") => {
                let Some((_, dirname)) = cd.split_once(" cd ") else {
                    return Err(anyhow!("Cannot split cd: {}", cd));
                };

                cwd.push(String::from(dirname) + "/");
                dirs.entry(cwd.join("")).or_default();
            }
            dir if dir.starts_with("dir ") => {
                let Some((_, dirname)) = dir.split_once(" ") else {
                    return Err(anyhow!("Cannot split dir: {}", dir));
                };

                let mut child_dir = cwd.clone();
                child_dir.push(String::from(dirname) + "/");

                dirs.entry(cwd.join(""))
                    .and_modify(|dir| dir.child_dirs.push(child_dir.join("")));
            }
            file if file.as_bytes()[0].is_ascii_digit() => {
                let Some((filesize, _)) = file.split_once(" ") else {
                    return Err(anyhow!("Cannot split file: {}", file));
                };

                let filesize = filesize.parse::<u32>()?;

                dirs.entry(cwd.join(""))
                    .and_modify(|dir| dir.size += filesize);
            }
            "$ ls" => (),
            _ => unreachable!(),
        }
    }

    Ok(dirs)
}

/// Calculates the size of each directory in dirs.
fn dir_sizes(dirs: &HashMap<String, Dir>) -> Result<HashMap<String, u32>> {
    let mut cache = HashMap::new();

    dirs.keys().for_each(|path| {
        dir_size(path, dirs, &mut cache);
    });

    Ok(cache)
}

/// Calculates the size of a directory and its children.
fn dir_size(path: &String, dirs: &HashMap<String, Dir>, cache: &mut HashMap<String, u32>) -> u32 {
    if cache.contains_key(path) {
        return cache[path];
    }

    let dir = &dirs[path];
    let dir_size = dir.size
        + dir
            .child_dirs
            .iter()
            .map(|child_dir| dir_size(child_dir, dirs, cache))
            .sum::<u32>();

    *cache.entry(path.clone()).or_insert(dir_size)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(EXAMPLE_INPUT.trim().to_string())?, 95437);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(EXAMPLE_INPUT.trim().to_string())?, 24933642);

        Ok(())
    }
}
