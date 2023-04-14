use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

enum FileSystemNode {
    File {
        name: String,
        size: u64,
    },
    Directory {
        name: String,
        children: HashMap<String, FileSystemNode>,
    },
}

impl FileSystemNode {
    fn new_directory(name: &str) -> Self {
        FileSystemNode::Directory {
            name: name.to_string(),
            children: HashMap::new(),
        }
    }

    fn new_file(name: &str, size: u64) -> Self {
        FileSystemNode::File {
            name: name.to_string(),
            size,
        }
    }

    fn dir_size(&self) -> u64 {
        match self {
            FileSystemNode::File { size, .. } => *size,
            FileSystemNode::Directory { children, .. } => {
                children.values().map(|child| child.dir_size()).sum()
            }
        }
    }
}

struct FileSystem {
    root: FileSystemNode,
}

impl FileSystem {
    fn new() -> Self {
        FileSystem {
            root: FileSystemNode::new_directory("/"),
        }
    }

    fn add_node(&mut self, path: &str, node: FileSystemNode) {
        let path = Path::new(path);
        let mut current_node = &mut self.root;

        for component in path.components().skip(1) {
            if let Some(component_str) = component.as_os_str().to_str() {
                current_node = match current_node {
                    FileSystemNode::Directory { children, .. } => children
                        .entry(component_str.to_string())
                        .or_insert_with(|| FileSystemNode::new_directory(component_str)),
                    FileSystemNode::File { .. } => return, // Invalid path, proper error handling should be added.
                };
            }
        }

        let new_name = match &node {
            FileSystemNode::File { name, .. } => name,
            FileSystemNode::Directory { name, .. } => name,
        };

        if let FileSystemNode::Directory { children, .. } = current_node {
            children.entry(new_name.to_string()).or_insert(node);
        }
    }

    #[allow(clippy::only_used_in_recursion)]
    fn traverse(&self, node: &FileSystemNode, depth: usize) {
        match node {
            FileSystemNode::File { name, size } => {
                println!(
                    "{:indent$}- {} (file, size={})",
                    "",
                    name,
                    size,
                    indent = depth * 2
                );
            }
            FileSystemNode::Directory { name, children } => {
                println!("{:indent$}- {} (dir)", "", name, indent = depth * 2);
                for child in children.values() {
                    self.traverse(child, depth + 2);
                }
            }
        }
    }

    fn print(&self) {
        self.traverse(&self.root, 0);
    }

    fn filtered_sum(&self, node: &FileSystemNode, max_size: u64) -> u64 {
        match node {
            FileSystemNode::File { .. } => 0,
            FileSystemNode::Directory { children, .. } => {
                let mut sum = 0;
                for child in children.values() {
                    let child_size = child.dir_size();
                    if let FileSystemNode::Directory { .. } = child {
                        if child_size <= max_size {
                            sum += child_size;
                        }
                    }
                    sum += self.filtered_sum(child, max_size);
                }
                sum
            }
        }
    }

    fn find_sum_of_dirs_with_max_size(&self, max_size: u64) -> u64 {
        self.filtered_sum(&self.root, max_size)
    }

    fn find_smallest_directory(
        &self,
        node: &FileSystemNode,
        min_size: u64,
    ) -> Option<(String, u64)> {
        match node {
            FileSystemNode::File { .. } => None,
            FileSystemNode::Directory { name, children } => {
                let mut smallest_directory: Option<(String, u64)> = None;

                for child in children.values() {
                    let child_size = child.dir_size();
                    if let FileSystemNode::Directory { name, .. } = child {
                        if child_size >= min_size {
                            if let Some((_, smallest_size)) = smallest_directory {
                                if child_size < smallest_size {
                                    smallest_directory = Some((name.clone(), child_size));
                                }
                            } else {
                                smallest_directory = Some((name.clone(), child_size));
                            }
                        }
                    }

                    if let Some((child_name, child_size)) =
                        self.find_smallest_directory(child, min_size)
                    {
                        if let Some((_, smallest_size)) = smallest_directory {
                            if child_size < smallest_size {
                                smallest_directory = Some((child_name, child_size));
                            }
                        } else {
                            smallest_directory = Some((child_name, child_size));
                        }
                    }
                }

                smallest_directory
            }
        }
    }
}

fn main() {
    let mut tree = FileSystem::new();

    // Read the input file line by line
    let input = read_to_string("input.txt").unwrap();
    let lines = input.lines();

    let mut current_path = String::from("/");

    // Skip the first line as it's just the command
    for line in lines.skip(1) {
        match line {
            // if the line starts with "$ ls" ignore it we need to output of the ls
            line if line.starts_with("$ ls") => {
                continue;
            }
            // If the line starts with a number, it's a file
            line if line.starts_with(char::is_numeric) => {
                let parts = line.split_whitespace().collect::<Vec<_>>();
                let size = parts[0].parse::<u64>().unwrap();
                let name = parts[1];
                tree.add_node(current_path.as_str(), FileSystemNode::new_file(name, size));
            }
            // If the line starts with "dir", it's a directory
            line if line.starts_with("dir") => {
                let dirname = line.split_whitespace().collect::<Vec<_>>()[1];
                tree.add_node(
                    current_path.as_str(),
                    FileSystemNode::new_directory(dirname),
                );
            }
            // If the line starts with "cd", it's a change directory command
            line if line.starts_with("$ cd") => {
                let dirname = line.split_whitespace().collect::<Vec<_>>()[2];

                // Check if the command is to go up one directory ($ cd ..)
                if dirname == ".." {
                    // Remove the last part of the path (go up one directory)
                    let mut path_components = current_path.split("/").collect::<Vec<_>>();
                    path_components.pop();
                    current_path = path_components.join("/");
                } else {
                    // Change the current path
                    current_path = format!("{}/{}", current_path, dirname);
                }
            }
            _ => {}
        }
    }

    tree.print();

    println!(
        "Sum of directories with size <= 100000: {}",
        tree.find_sum_of_dirs_with_max_size(100000)
    );

    let total_disk_space = 70_000_000;
    let required_unused_space = 30_000_000;
    let used_space = tree.root.dir_size();
    let current_unused_space = total_disk_space - used_space;
    let additional_space_needed = required_unused_space - current_unused_space;

    if let Some((smallest_dir, size)) =
        tree.find_smallest_directory(&tree.root, additional_space_needed)
    {
        println!("Smallest directory to delete: {} size: {}", smallest_dir, size);
    } else {
        println!("No directory can be deleted to free up enough space.");
    }
}
