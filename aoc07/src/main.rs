use std::{error::Error, fs};

use slab_tree::{NodeRef, Tree, TreeBuilder};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let tree = parse_term(&input);
    println!("Part 1: {:?}", part_1_sum_dirs_size_lt_100_000(&tree));
    Ok(())
}

fn part_1_sum_dirs_size_lt_100_000(tree: &Tree<FileNode>) -> usize {
    tree.root()
        .unwrap()
        .traverse_level_order()
        .filter_map(|x| match x.data() {
            FileNode::Dir(_) => {
                let size = size(&x);
                if size < 100_000 {
                    Some(size)
                } else {
                    None
                }
            }
            FileNode::File(_, _) => None,
        })
        .sum()
}

fn size(node: &NodeRef<FileNode>) -> usize {
    match node.data() {
        FileNode::Dir(_) => node.children().map(|c| size(&c)).sum(),
        FileNode::File(size, _) => *size,
    }
}

fn parse_term(output: &str) -> Tree<FileNode> {
    let mut tree = TreeBuilder::new()
        .with_root(FileNode::Dir("/".to_string()))
        .build();

    let mut work_dir_id = tree.root().unwrap().node_id();

    for cmd in output.split("\n$ ").skip(1) {
        if cmd == "cd .." {
            work_dir_id = tree.get(work_dir_id).unwrap().parent().unwrap().node_id();
        } else if cmd.starts_with("cd") {
            let (_, cd_to) = cmd.split_once(" ").unwrap();
            work_dir_id = tree
                .get(work_dir_id)
                .unwrap()
                .children()
                .find(|x| *x.data() == FileNode::Dir(cd_to.to_string()))
                .unwrap()
                .node_id();
        } else if cmd.starts_with("ls") {
            let mut work_dir = tree.get_mut(work_dir_id).unwrap();
            for ls_out in cmd.lines().skip(1) {
                if ls_out.starts_with("dir") {
                    let (_, dir_name) = ls_out.split_once(" ").unwrap();
                    work_dir.append(FileNode::Dir(dir_name.to_string()));
                } else {
                    let (size, name) = ls_out.split_once(" ").unwrap();
                    work_dir.append(FileNode::File(size.parse().unwrap(), name.to_string()));
                }
            }
        } else {
            unreachable!();
        }
    }
    tree
}

#[derive(Clone, Debug, PartialEq)]
enum FileNode {
    Dir(String),
    File(usize, String),
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_TERMINAL_OUTPUT: &'static str = "$ cd /
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
    fn it_parses_terminal_output() {
        parse_term(TEST_TERMINAL_OUTPUT);
    }

    #[test]
    fn it_passes_example_1() {
        assert_eq!(
            95437,
            part_1_sum_dirs_size_lt_100_000(&parse_term(TEST_TERMINAL_OUTPUT))
        );
    }
}
