#[derive(Debug)]
enum Cmd {
    Ls,
    Cd(String),
    Dir(String),
    File((usize, String)),
}

fn parse_commands(input: &str) -> Vec<Cmd> {
    input
        .lines()
        .map(|l| {
            if l.starts_with("$ ls") {
                return Cmd::Ls;
            } else if l.starts_with("$ cd") {
                let name = l.strip_prefix("$ cd ").unwrap().to_string();
                return Cmd::Cd(name);
            } else if l.starts_with("dir") {
                let name = l.strip_prefix("dir ").unwrap().to_string();
                return Cmd::Dir(name);
            }

            // file
            let mut parts = l.split_whitespace();

            let size = parts.next().unwrap().parse::<usize>().unwrap();
            let name = parts.next().unwrap().to_string();
            Cmd::File((size, name))
        })
        .collect::<Vec<Cmd>>()
}

#[derive(Debug)]
struct FsEntry {
    path: Vec<String>,
    dir: bool,
    size: usize,
}

fn parse_entries(commands: Vec<Cmd>) -> Vec<FsEntry> {
    let mut current_path = vec![];
    let mut entries = vec![];

    for c in commands {
        match c {
            Cmd::Cd(n) => {
                if n == ".." {
                    current_path.pop();
                } else {
                    current_path.push(n);
                }
            }
            Cmd::File((s, n)) => {
                let mut path = current_path.clone();
                path.push(n);
                entries.push(FsEntry {
                    path,
                    dir: false,
                    size: s,
                });
            }
            Cmd::Dir(n) => {
                let mut path = current_path.clone();
                path.push(n);
                entries.push(FsEntry {
                    path,
                    dir: true,
                    size: 0,
                });
            }
            _ => {}
        }
    }

    entries
}

pub fn puzzle_1(input: &str) -> String {
    let commands = parse_commands(input);
    let entries = parse_entries(commands);

    let dir_sizes: Vec<usize> = entries
        .iter()
        .filter_map(|e| {
            if !e.dir {
                return None;
            }

            let parent_path = e.path.as_slice();
            // get the directory size from all descendent files
            let size = entries
                .iter()
                .filter_map(|entry| {
                    // ignore folders and files without the parent path
                    if entry.dir || !entry.path.starts_with(parent_path) {
                        return None;
                    }

                    Some(entry.size)
                })
                .sum::<usize>();

            Some(size)
        })
        .collect();

    // sum all folder sizes which fulfill the requirements
    let result = dir_sizes
        .iter()
        .filter(|s| {
            if **s <= 100000 {
                return true;
            }
            false
        })
        .sum::<usize>();

    result.to_string()
}

const TOTAL: usize = 70000000;
const REQUIRED: usize = 30000000;

pub fn puzzle_2(input: &str) -> String {
    let commands = parse_commands(input);
    let entries = parse_entries(commands);

    let used_space = entries.iter().map(|e| e.size).sum::<usize>();
    let free_space = TOTAL - used_space;
    let minimum = REQUIRED - free_space;

    let dir_sizes: Vec<usize> = entries
        .iter()
        .filter_map(|e| {
            if !e.dir {
                return None;
            }

            let parent_path = e.path.as_slice();
            // get the directory size from all descendent files
            let size = entries
                .iter()
                .filter_map(|entry| {
                    // ignore folders and files without the parent path
                    if entry.dir || !entry.path.starts_with(parent_path) {
                        return None;
                    }

                    Some(entry.size)
                })
                .sum::<usize>();

            Some(size)
        })
        .collect();

    // find the smallest folder which fulfills the requirements
    let result = dir_sizes
        .iter()
        .filter(|s| {
            if **s >= minimum {
                return true;
            }
            false
        })
        .min()
        .unwrap();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
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
7214296 k";

    #[test]
    fn p1() {
        let result = puzzle_1(INPUT);
        assert_eq!(result, "95437");
    }

    #[test]
    fn p2() {
        let result = puzzle_2(INPUT);
        assert_eq!(result, "24933642");
    }
}
