use std::{cell::RefCell, rc::Rc};

use crate::Problem;

trait Visitor {
    fn visit_file_tree(&mut self, node: &FileTree);
}

struct FindDirectoriesOfSize {
    size: usize,
    total_size: usize,
}

impl FindDirectoriesOfSize {
    fn new(size: usize) -> Self {
        FindDirectoriesOfSize {
            size,
            total_size: 0,
        }
    }
}
impl Visitor for FindDirectoriesOfSize {
    fn visit_file_tree(&mut self, node: &FileTree) {
        if matches!(node, FileTree::Directory(_)) {
            let s = node.size_on_disk();
            if s <= self.size {
                self.total_size += s;
            }
        }
    }
}

struct FindDirectoriesLargerThan {
    size: usize,
    smallest_dir: Option<usize>,
}

impl FindDirectoriesLargerThan {
    fn new(size: usize) -> Self {
        Self {
            size,
            smallest_dir: None,
        }
    }
}
impl Visitor for FindDirectoriesLargerThan {
    fn visit_file_tree(&mut self, node: &FileTree) {
        if matches!(node, FileTree::Directory(_)) {
            let s = node.size_on_disk();
            if s >= self.size && self.smallest_dir.map_or(true, |d| s < d) {
                self.smallest_dir = Some(s);
            }
        }
    }
}

#[derive(Debug)]
struct Directory {
    name: String,
    children: RefCell<Vec<Rc<FileTree>>>,
    size_on_disk: RefCell<Option<usize>>,
}

impl Directory {
    fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            children: RefCell::new(vec![]),
            size_on_disk: RefCell::new(None),
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn size_on_disk(&self) -> usize {
        let mut size_on_disk = self.size_on_disk.borrow_mut();
        match *size_on_disk {
            Some(s) => s,

            None => {
                let s = self
                    .children
                    .borrow()
                    .iter()
                    .map(|n| n.size_on_disk())
                    .sum();
                *size_on_disk = Some(s);
                s
            }
        }
    }
}

#[derive(Debug)]
enum FileTree {
    Directory(Directory),
    File(String, usize),
}

impl FileTree {
    fn new_directory<S: Into<String>>(name: S) -> FileTree {
        FileTree::Directory(Directory::new(name))
    }

    fn name(&self) -> &str {
        match self {
            FileTree::Directory(d) => d.name(),
            FileTree::File(n, _) => n,
        }
    }

    fn add_node(&self, node: FileTree) {
        match self {
            FileTree::Directory(d) => d.children.borrow_mut().push(Rc::new(node)),
            FileTree::File(_, _) => panic!("files can't contain nodes"),
        }
    }

    fn directory(&self, name: &str) -> Option<Rc<FileTree>> {
        match self {
            FileTree::Directory(d) => d
                .children
                .borrow()
                .iter()
                .find(|n| n.name() == name)
                .map(Rc::clone),
            FileTree::File(_, _) => panic!("files can't contain nodes"),
        }
    }

    fn size_on_disk(&self) -> usize {
        match self {
            FileTree::Directory(d) => d.size_on_disk(),
            FileTree::File(_, s) => *s,
        }
    }

    fn visit<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_file_tree(self);

        if let FileTree::Directory(d) = self {
            for n in d.children.borrow().iter() {
                n.visit(visitor);
            }
        }
    }
}

fn read_file_tree(lines: &[String]) -> Rc<FileTree> {
    let root = FileTree::Directory(Directory::new("/"));
    let mut stack = RefCell::new(vec![Rc::new(root)]);

    // skip the first one, starting in root directory
    for l in lines.iter().skip(1) {
        let mut parts = l.split(' ');
        match parts.next().unwrap() {
            "$" => {
                let command = parts.next().unwrap();
                match command {
                    "cd" => {
                        let dirname = parts.next().unwrap();
                        match dirname {
                            ".." => {
                                stack.borrow_mut().pop();
                            }
                            dirname => {
                                let mut stack = stack.borrow_mut();
                                let node = stack.last().unwrap().directory(dirname).unwrap();
                                stack.push(node);
                            }
                        };
                    }
                    "ls" => { /* we don't have to do anything */ }
                    _ => todo!("not implemented yet"),
                }
            }
            "dir" => stack
                .borrow_mut()
                .last_mut()
                .unwrap()
                .add_node(FileTree::new_directory(parts.next().unwrap().to_owned())),
            v => stack
                .borrow_mut()
                .last_mut()
                .unwrap()
                .add_node(FileTree::File(
                    parts.next().unwrap().to_owned(),
                    v.parse().unwrap(),
                )),
        }
    }

    stack.get_mut().remove(0)
}

pub struct Problem7;
impl Problem for Problem7 {
    fn solve_part1(&mut self, lines: &[String]) -> String {
        let tree = read_file_tree(lines);
        let mut v = FindDirectoriesOfSize::new(100000);
        tree.visit(&mut v);
        v.total_size.to_string()
    }

    fn solve_part2(&mut self, lines: &[String]) -> String {
        const DISK_SIZE: usize = 70_000_000;
        const REQUIRED_SPACE: usize = 30_000_000;
        let tree = read_file_tree(lines);

        let free_space = DISK_SIZE - tree.size_on_disk();
        let mut v = FindDirectoriesLargerThan::new(REQUIRED_SPACE - free_space);
        tree.visit(&mut v);
        v.smallest_dir.unwrap().to_string()
    }
}
