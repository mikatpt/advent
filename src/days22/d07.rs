use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{get_input, Result};

pub fn solve() -> Result<(usize, usize)> {
    let input = get_input(7)?;

    Ok((part1(&input).unwrap(), part2(&input).unwrap()))
}

type NodePointer = Rc<RefCell<TreeNode>>;

#[derive(Debug, Default)]
struct TreeNode {
    name: String,
    size: usize,
    kids: HashMap<String, NodePointer>,
    parent: Option<NodePointer>,
    is_dir: bool,
}

impl TreeNode {
    fn new(name: &str, is_dir: bool, parent: Option<NodePointer>) -> Self {
        TreeNode {
            name: name.to_string(),
            is_dir,
            parent,
            ..Default::default()
        }
    }

    fn from_input(input: &str) -> Option<Self> {
        let root = TreeNode::new("/", true, None);
        let mut lines = input.lines().peekable();

        let head = Rc::new(RefCell::new(root));
        let mut node = head.clone();
        lines.next();

        while let Some(line) = lines.next() {
            if !line.starts_with('$') {
                continue;
            }
            let cmd = &line[2..4];

            match cmd {
                "ls" => {
                    // consume all non-commands.
                    while let Some(line) = lines.peek() && !line.starts_with('$') {
                        let line = lines.next()?;
                        let (dir_or_size, name) = line.split_once(' ')?;

                        let mut new_node = TreeNode::new(name, dir_or_size == "dir", Some(node.clone()));
                        if dir_or_size != "dir" {
                            let size = dir_or_size.parse::<usize>().expect("this should be a number");
                            new_node.size = size;
                        };
                        node.borrow_mut().kids.insert(name.to_string(), Rc::new(RefCell::new(new_node)));
                    }
                }
                "cd" => {
                    let next_dir = &line[5..];
                    let prev = node.clone();
                    let borrowed = prev.borrow();

                    node = match next_dir {
                        ".." => borrowed.parent.as_ref()?.clone(),
                        _ => borrowed.kids.get(next_dir)?.clone(),
                    };
                }
                _ => unreachable!("no other commands besides ls and cd"),
            }
        }
        let mut tree = head.take();
        tree.set_sizes();
        Some(tree)
    }

    fn set_sizes(&mut self) -> usize {
        let mut sum = self.size;

        for kid in self.kids.values() {
            let recurse = kid.borrow_mut().set_sizes();
            if self.is_dir {
                sum += recurse;
            }
        }
        if self.is_dir {
            self.size = sum;
        }

        sum
    }

    fn sum_largest_dirs(&self) -> usize {
        self.get_dir_sizes()
            .iter()
            .fold(0, |sum, &size| sum + if size <= 100000 { size } else { 0 })
    }

    fn find_dirs_to_delete(&self) -> usize {
        let unused_disk_space = 70000000 - self.size;
        let deficit = 30000000 - unused_disk_space;

        self.get_dir_sizes().iter().fold(usize::MAX, |min, &dir| {
            std::cmp::min(min, if dir >= deficit { dir } else { usize::MAX })
        })
    }

    fn get_dir_sizes(&self) -> Vec<usize> {
        let mut res = vec![];

        if self.is_dir && self.name != "/" {
            res.push(self.size);
        }

        for kid in self.kids.values() {
            res.extend(kid.borrow().get_dir_sizes())
        }

        res
    }
}

fn part1(input: &str) -> Option<usize> {
    let tree = TreeNode::from_input(input)?;
    let sum = tree.sum_largest_dirs();

    Some(sum)
}

fn part2(input: &str) -> Option<usize> {
    let tree = TreeNode::from_input(input)?;
    let sum = tree.find_dirs_to_delete();
    Some(sum)
}

const INPUT: &str = "$ cd /
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing() {
        crate::trace::init();
        assert_eq!(95437, part1(INPUT).unwrap());
        assert_eq!(24933642, part2(INPUT).unwrap());
    }
}
