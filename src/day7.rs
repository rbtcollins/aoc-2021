use std::{cmp::min, collections::HashMap};

use aoc_runner_derive::{aoc, aoc_generator};

trait FS {
    fn size(&self) -> usize;
}

#[derive(Default, Clone, Debug)]
struct File(usize);

impl FS for File {
    fn size(&self) -> usize {
        self.0
    }
}

#[derive(Default, Clone, Debug)]
struct Dir {
    files: Vec<File>,
    dirs: HashMap<String, Dir>,
}

impl FS for Dir {
    fn size(&self) -> usize {
        self.files.iter().map(|c| c.size()).sum::<usize>()
            + self.dirs.values().map(|d| d.size()).sum::<usize>()
    }
}

impl Dir {
    fn find_child(&mut self, path: &[&str]) -> &mut Dir {
        if path.is_empty() {
            return self;
        }

        self.dirs.get_mut(path[0]).unwrap().find_child(&path[1..])
    }
}

#[aoc_generator(day7)]
fn generate(input: &str) -> Dir {
    // quadratic because traverses from root; shared ownership is gnarly in
    // rust and can't be bothered mucking with a RefCell etc.
    input
        .lines()
        .fold((vec![], Dir::default()), |(mut cwd, mut root), e| {
            let symbols = e.split_ascii_whitespace().collect::<Vec<_>>();
            match &symbols[..] {
                ["$", "cd", "/"] => {
                    cwd = vec![];
                }
                ["$", "cd", ".."] => {
                    cwd.pop();
                }
                ["$", "cd", name] => {
                    cwd.push(name.to_owned());
                }
                ["$", "ls"] => (),
                ["dir", name] => {
                    root.find_child(&cwd)
                        .dirs
                        .insert(name.to_string(), Dir::default());
                }
                [size, _name] => {
                    root.find_child(&cwd)
                        .files
                        .push(File(str::parse(size).unwrap()));
                }
                x => unreachable!("unreachable {x:?}"),
            }
            (cwd, root)
        })
        .1
}

#[aoc(day7, part1)]
fn part1(input: &Dir) -> usize {
    let mut queue = vec![input];
    let mut acc = 0;
    while !queue.is_empty() {
        let next = queue.pop().unwrap();
        let next_size = next.size();
        if next_size <= 100000 {
            acc += next_size;
        }
        queue.extend(next.dirs.values());
    }
    acc
}

#[aoc(day7, part2)]
fn part2(input: &Dir) -> usize {
    let device_size = 70000000;
    let update_size = 30000000;
    let used_size = input.size();
    let unused_space = device_size - used_size;
    let min_to_free = update_size - unused_space;
    let mut queue = vec![input];
    let mut smallest_size = device_size;
    while !queue.is_empty() {
        let next = queue.pop().unwrap();
        let next_size = next.size();
        if next_size > min_to_free {
            smallest_size = min(next.size(), smallest_size);
            // smaller than threshold dirs aren't worth exploring children of.
            queue.extend(next.dirs.values());
        }
    }
    smallest_size
}

#[cfg(test)]
mod tests {

    const SAMPLE: &str = r#"$ cd /
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
"#;

    #[test]
    fn part1() {
        let input = super::generate(SAMPLE);
        assert_eq!(95437, super::part1(&input));
    }

    #[test]
    fn part2() {
        let input = super::generate(SAMPLE);
        assert_eq!(24933642, super::part2(&input));
    }
}
