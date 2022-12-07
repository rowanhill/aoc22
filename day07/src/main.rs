use std::collections::HashMap;

struct FsTree {
    nodes: Vec<FsNode>,
    dir_size_cache: HashMap<usize, u64>,
}
enum FsNode {
    Dir(Option<usize>, HashMap<String, usize>),
    File(u64)
}

impl FsTree {
    fn new() -> FsTree {
        FsTree {
            nodes: vec![FsNode::Dir(None, HashMap::new())],
            dir_size_cache: HashMap::new(),
        }
    }

    fn add_child<F>(&mut self, parent_index: usize, name: &str, create_node: F) -> usize where F: Fn() -> FsNode {
        let parent = self.nodes.get(parent_index);
        if let Some(FsNode::Dir(_, children)) = parent {
            if children.contains_key(name) {
                return *children.get(name).unwrap();
            }
        } else {
            panic!("Node {} is not Dir", parent_index);
        }

        let new_index = self.nodes.len();
        self.nodes.push(create_node());
        let parent = &mut self.nodes[parent_index];
        if let FsNode::Dir(_, children) = parent {
            children.insert(name.to_string(), new_index);
        } else {
            panic!("Node {} is not Dir", parent_index);
        }
        new_index
    }

    fn dir(&mut self, parent_index: usize, name: &str) -> usize {
        self.add_child(parent_index, name, || FsNode::Dir(Some(parent_index), HashMap::new()))
    }

    fn file(&mut self, parent_index: usize, name: &str, size: u64) -> usize {
        self.add_child(parent_index, name, || FsNode::File(size))
    }

    fn parent(&self, cwd: usize) -> usize {
        if let FsNode::Dir(Some(parent), _) = &self.nodes[cwd] {
            *parent
        } else {
            panic!("Node {} is not Dir", cwd);
        }
    }

    fn calc_dir_size(&mut self, dir_index: usize) -> u64 {
        if let Some(size) = self.dir_size_cache.get(&dir_index) {
            return *size;
        }

        let mut dir_indexes = vec![];
        if let FsNode::Dir(_, children) = &self.nodes[dir_index] {
            let mut size = 0;
            for &child_index in children.values() {
                match &self.nodes[child_index] {
                    FsNode::File(s) => size += *s,
                    FsNode::Dir(_, _) => dir_indexes.push(child_index),
                };
            }
            for child_index in dir_indexes {
                size += self.calc_dir_size(child_index);
            }
            self.dir_size_cache.insert(dir_index, size);
            size
        } else {
            panic!("Node {} is not Dir", dir_index);
        }
    }

    fn sum_of_dirs_not_more_than(&self, limit: u64) -> u64 {
        self.dir_size_cache.values()
            .filter(|s| s <= &&limit)
            .sum()
    }

    fn smallest_directory_freeing_up_to(&self, target: u64) -> u64 {
        let total_size = 70000000;
        let cur_free = total_size - self.dir_size_cache[&0];
        let min_dir_size = target - cur_free;
        *self.dir_size_cache.values()
            .filter(|s| s >= &&min_dir_size)
            .min()
            .unwrap()
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut tree = FsTree::new();
    let mut cwd = 0;
    for line in input.lines() {
        if line == "$ cd /" {
            cwd = 0;
        } else if line == "$ cd .." {
            cwd = tree.parent(cwd);
        } else if line.starts_with("$ cd ") {
            let name = &line[5..];
            cwd = tree.dir(cwd, name);
        } else if !line.starts_with("$") && !line.starts_with("dir") {
            let (size, name) = line.split_once(" ").unwrap();
            let size: u64 = size.parse().unwrap();
            tree.file(cwd, name, size);
        }
    }
    tree.calc_dir_size(0);
    println!("Part 1: {}", tree.sum_of_dirs_not_more_than(100000));
    println!("Part 2: {}", tree.smallest_directory_freeing_up_to(30000000));
}