// AOC Day 7 2022 - Rust

use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct File {
    name: String,
    size: u32,
}

#[derive(Debug)]
struct Folder {
    name: String,
    files: Vec<File>,
    folders: Vec<Rc<RefCell<Folder>>>,
    parent: Option<Rc<RefCell<Folder>>>,
}

type FileSystem = Folder;

impl Folder {
    fn new(name: String) -> Self {
        Self {
            name,
            files: vec![],
            folders: vec![],
            parent: None,
        }
    }

    fn add_file(&mut self, file: File) {
        self.files.push(file);
    }

    fn add_folder(&mut self, folder: Rc<RefCell<Folder>>) {
        self.folders.push(folder);
    }

    fn calculate_size(&self) -> u32 {
        let mut size = 0;
        self.files.iter().for_each(|file| {
            size += file.size;
        });
        self.folders.iter().for_each(|folder| {
            size += folder.borrow().calculate_size();
        });
        size
    }
}

fn total_size_folders_larger_than(minimum_size: u32, folder: &Rc<RefCell<FileSystem>>) -> u32 {
    let mut total_size = 0;

    let size = folder.borrow().calculate_size();
    if size <= minimum_size {
        total_size += size;
    }
    folder.borrow().folders.iter().for_each(|folder| {
        total_size += total_size_folders_larger_than(minimum_size, folder);
    });
    total_size
}

fn folder_sizes_larger_than(minimum_size: u32, folder: &Rc<RefCell<FileSystem>>) -> Vec<u32> {
    let mut folders = vec![];

    let size = folder.borrow().calculate_size();
    if size >= minimum_size {
        folders.push(size);
    }
    folder.borrow().folders.iter().for_each(|folder| {
        folders.extend(folder_sizes_larger_than(minimum_size, folder));
    });
    folders
}

fn size_of_smallest_folder_larger_than(minimum_size: u32, folder: &Rc<RefCell<Folder>>) -> u32 {
    let mut sizes = vec![];

    if folder.borrow().calculate_size() >= minimum_size {
        sizes.push(folder.borrow().calculate_size());
    }

    folder.borrow().folders.iter().for_each(|folder| {
        sizes.extend(folder_sizes_larger_than(minimum_size, folder));
    });

    sizes.sort();
    sizes[0]
}
fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Unable to read file");

    let root = Rc::new(RefCell::new(FileSystem {
        name: String::from("root"),
        files: Vec::new(),
        folders: Vec::new(),
        parent: None,
    }));

    build_tree(input, &root);

    match std::env::var("part").unwrap_or_default().as_str() {
        "part2" => print!("{}", part_two(&root)),
        _ => println!("{}", total_size_folders_larger_than(100000, &root)),
    };
}

fn part_two(root: &Rc<RefCell<Folder>>) -> u32 {
    let space_available = 70000000 - root.borrow().calculate_size();
    let space_to_delete = 30000000 - space_available;
    let result = size_of_smallest_folder_larger_than(space_to_delete, &root);
    result
}

fn build_tree(input: String, root: &Rc<RefCell<Folder>>) -> &Rc<RefCell<Folder>> {
    let commands = input
        .split("$")
        .map(|line| line.lines().map(|s| s.trim()).collect::<Vec<&str>>())
        .filter(|command| !command.is_empty());

    let mut current_folder = root.clone();
    commands.for_each(|command| match &command[..] {
        ["cd /"] => cd_to(&mut current_folder, root),
        ["cd .."] => cd_to_parent(&mut current_folder),
        [cd_command] => {
            cd_to_x(cd_command, &mut current_folder);
        }
        ["ls", contents @ ..] => {
            register_content(contents, &mut current_folder);
        }
        _ => println!("Unknown command: {:?}", command),
    });
    return root;
}

fn register_content(contents: &[&str], current_folder: &mut Rc<RefCell<Folder>>) {
    contents.iter().for_each(|entry| {
        if entry.starts_with(char::is_numeric) {
            let (size, name) = entry.split_once(" ").expect("Couldnt split file");
            current_folder.borrow_mut().add_file(File {
                name: name.to_string(),
                size: size.parse().expect("Couldnt parse size"),
            })
        } else {
            let (_, name) = entry.split_once(" ").expect("Couldnt split folder");
            let folder = Rc::new(RefCell::new(Folder::new(name.to_string())));
            current_folder.borrow_mut().add_folder(folder.clone());
            folder.borrow_mut().parent = Some(current_folder.clone());
        }
    });
}

fn cd_to_x(cd_command: &&str, current_folder: &mut Rc<RefCell<Folder>>) {
    let (_, folder_name) = cd_command
        .split_once(" ")
        .expect("Couldnt split cd command");
    let children = current_folder.borrow().folders.clone();
    let target_folder = children
        .iter()
        .find(|folder| folder.borrow().name == folder_name.to_string())
        .expect("Couldnt find folder");
    cd_to(current_folder, target_folder);
}
fn cd_to_parent(current_folder: &mut Rc<RefCell<Folder>>) {
    let parent = current_folder.borrow().parent.clone();
    if let Some(parent) = parent {
        *current_folder = parent;
    }
}

fn cd_to(current_folder: &mut Rc<RefCell<Folder>>, target_folder: &Rc<RefCell<Folder>>) {
    *current_folder = target_folder.clone()
}
