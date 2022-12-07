use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::FsNode::{Dir, File};

type NodeRef = Rc<RefCell<FsNode>>;

enum FsNode {
    Dir(Option<NodeRef>, HashMap<String, NodeRef>, u64),
    File(u64)
}

impl FsNode {
    fn new_dir(parent: Option<NodeRef>) -> FsNode {
        Dir(parent, HashMap::new(), 0)
    }

    fn size(&self) -> u64 {
        match self {
            Dir(_, _, s) => *s,
            File(s) => *s
        }
    }

    fn parent(&self) -> NodeRef {
        match self {
            Dir(Some(parent), _, _) => Rc::clone(parent),
            Dir(_, _, _) => panic!("Root has no parent"),
            _ => panic!("Non-Dir cannot get parent"),
        }
    }

    fn add_subdir(&mut self, name: &str, parent: NodeRef) -> NodeRef {
        if let Dir(_, children, _) = self {
            let subdir = Rc::new(RefCell::new(FsNode::new_dir(Some(parent))));
            children.insert(name.to_string(), Rc::clone(&subdir));
            subdir
        } else {
            panic!("Cannot add subdir to File");
        }
    }

    fn add_file(&mut self, name: &str, size: u64) -> NodeRef {
        if let Dir(parent, children, dir_size) = self {
            let file = Rc::new(RefCell::new(File(size)));

            children.insert(name.to_string(), Rc::clone(&file));
            *dir_size += size;

            let mut cur_dir = parent.clone();
            while let Some(next_parent_ref) = cur_dir {
                let mut next_parent_ref_mut = next_parent_ref.borrow_mut();
                if let Dir(next_dir, _, dir_size) = &mut *next_parent_ref_mut {
                    *dir_size += size;
                    cur_dir = next_dir.clone();
                } else {
                    break;
                }
            }
            file
        } else {
            panic!("Cannot add file to File");
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let root = Rc::new(RefCell::new(FsNode::new_dir(None)));
    let mut cwd = Rc::clone(&root);

    for line in input.lines() {
        if line == "$ cd /" {
            cwd = Rc::clone(&root);
        } else if line == "$ cd .." {
            let parent = cwd.borrow().parent();
            cwd = Rc::clone(&parent);
        } else if line.starts_with("$ cd ") {
            let name = &line[5..];
            let subdir = cwd.borrow_mut().add_subdir(name, Rc::clone(&cwd));
            cwd = subdir;
        } else if !line.starts_with("$") && !line.starts_with("dir") {
            let (size, name) = line.split_once(" ").unwrap();
            let size: u64 = size.parse().unwrap();
            cwd.borrow_mut().add_file(name, size);
        }
    }

    let pt1 = sum_small_dirs(Rc::clone(&root));
    println!("Part 1: {}", pt1);

    let root_size = root.borrow().size();
    let free_space = 70_000_000 - root_size;
    let required_space = 30_000_000 - free_space;

    let pt2 = smallest_dir_above_threshold(Rc::clone(&root), required_space, root_size);
    println!("Part 2: {}", pt2.unwrap());
}

fn sum_small_dirs(node: NodeRef) -> u64 {
    let mut sum = 0;
    if let Dir(_, children, size) = &*node.borrow() {
        if size <= &100_000 {
            sum += size;
        }
        sum += children.values().map(|child| {
            sum_small_dirs(Rc::clone(&child))
        }).sum::<u64>();
    }
    sum
}

fn smallest_dir_above_threshold(node: NodeRef, threshold: u64, cur_smallest: u64) -> Option<u64> {
    if let Dir(_, children, size) = &*node.borrow() {
        let mut new_smallest = if size < &cur_smallest && size >= &threshold { *size } else { cur_smallest };
        for child in children.values() {
            if let Some(s) = smallest_dir_above_threshold(Rc::clone(&child), threshold, new_smallest) {
                if s < new_smallest && s >= threshold {
                    new_smallest = s;
                }
            }
        }
        Some(new_smallest)
    } else {
        None
    }
}