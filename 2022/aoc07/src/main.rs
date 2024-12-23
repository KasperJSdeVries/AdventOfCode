#![allow(dead_code)]

use std::fmt::Debug;
use std::io::Read;
use std::rc::{Rc, Weak};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

trait FileSystemSize: Debug {
    fn get_size(&self) -> usize;
}

#[derive(Debug)]
struct Directory {
    name: String,
    parent: Option<Weak<Directory>>,
    files: Mutex<Vec<File>>,
    child_dirs: Mutex<Vec<Rc<Directory>>>,
}

impl Directory {
    fn new(name: impl Into<String>) -> Rc<Self> {
        Rc::new(Self {
            name: name.into(),
            parent: None,
            files: Mutex::new(Vec::new()),
            child_dirs: Mutex::new(Vec::new()),
        })
    }

    fn new_with_parent(name: impl Into<String>, parent: &Rc<Directory>) -> Rc<Self> {
        Rc::new(Self {
            name: name.into(),
            parent: Some(Rc::downgrade(parent)),
            files: Mutex::new(Vec::new()),
            child_dirs: Mutex::new(Vec::new()),
        })
    }
}

impl FileSystemSize for Directory {
    fn get_size(&self) -> usize {
        let mut size = 0;
        for child in &*self.child_dirs.lock().unwrap() {
            size += child.get_size();
        }
        for child in &*self.files.lock().unwrap() {
            size += child.get_size();
        }
        size
    }
}

#[derive(Debug)]
struct File {
    parent: Weak<Directory>,
    size: usize,
}

impl File {
    fn new(parent: &Rc<Directory>, size: usize) -> Self {
        Self {
            parent: Rc::downgrade(parent),
            size,
        }
    }
}

impl FileSystemSize for File {
    fn get_size(&self) -> usize {
        self.size
    }
}

const TOTAL_SIZE: usize = 70_000_000;
const SIZE_REQUIRED: usize = 30_000_000;

fn dir_can_be_deleted(dir: Rc<Directory>, current_fs_size: usize) -> bool {
    let size = dir.get_size();
    if TOTAL_SIZE - current_fs_size + size > SIZE_REQUIRED {
        return true;
    }
    false
}

fn get_deletable_dir_list(
    dir: Rc<Directory>,
    current_fs_size: usize,
    deletable_dirs: &mut Vec<Rc<Directory>>,
) {
    if dir_can_be_deleted(dir.clone(), current_fs_size) {
        deletable_dirs.push(dir.clone());

        for child_dir in &*dir.child_dirs.lock().unwrap() {
            get_deletable_dir_list(child_dir.clone(), current_fs_size, deletable_dirs);
        }
    }
}

fn main() {
    let mut input = String::new();

    {
        use std::fs::File;
        let mut file = File::open("input.txt").unwrap();
        file.read_to_string(&mut input).unwrap();
    }

    let root = Directory::new("/");
    let mut current_dir = root.clone();

    let mut lines = input.lines().peekable();
    'outer: loop {
        let Some(line) = lines.next() else {
            break;
        };

        if !line.starts_with("$") {
            panic!("Something went wrong in processing the commands");
        }

        let mut command = line.split_whitespace();
        command.next(); // skip the "$"
        match command.next().unwrap() {
            "cd" => match command.next().unwrap() {
                "/" => {
                    current_dir = root.clone();
                }
                ".." => {
                    if let Some(parent) = &current_dir.parent {
                        current_dir = parent.upgrade().unwrap();
                    }
                }
                dir => {
                    let temp = current_dir
                        .child_dirs
                        .lock()
                        .unwrap()
                        .iter()
                        .find(|&p| p.name == dir)
                        .unwrap()
                        .clone();
                    current_dir = temp;
                }
            },
            "ls" => loop {
                let Some(peek) = lines.peek() else {
                        break 'outer;
                    };

                if peek.starts_with("$") {
                    break;
                }

                let mut line = lines.next().unwrap().split_whitespace();
                match line.next().unwrap() {
                    "dir" => {
                        current_dir
                            .child_dirs
                            .lock()
                            .unwrap()
                            .push(Directory::new_with_parent(
                                line.next().unwrap(),
                                &current_dir,
                            ))
                    }
                    size => current_dir
                        .files
                        .lock()
                        .unwrap()
                        .push(File::new(&current_dir, size.parse().unwrap())),
                }
            },
            _ => panic!("Unknown command"),
        }
    }

    let mut deletable_dirs = Vec::new();
    get_deletable_dir_list(root.clone(), root.get_size(), &mut deletable_dirs);
    deletable_dirs.sort_by(|a, b| a.get_size().partial_cmp(&b.get_size()).unwrap());
    let result = deletable_dirs.first().unwrap();
    println!(
        "{}: {} -> {}",
        result.name,
        result.get_size(),
        root.get_size() - result.get_size()
    );
}
