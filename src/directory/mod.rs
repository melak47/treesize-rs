use std::cmp::Ordering;
use std::fs;
use std::path::Path;

pub mod tree;
pub mod print;
use self::tree::{DirectoryNode, FileNode, FSNode};

pub fn read_recursive(path: &String) -> FSNode {
    let path = fs::canonicalize(Path::new(&path)).unwrap();
    let mut node = DirectoryNode::new(path.file_name().unwrap().to_str().unwrap().to_string());

    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let meta = entry.metadata().unwrap();
        let name = entry.file_name().to_str().unwrap().to_string();

        if meta.is_file() {
            node.children.push(FSNode::File(FileNode {
                name: name,
                size: meta.len(),
            }));
            node.size += meta.len();
        } else if meta.is_dir() {
            let path = entry.path().to_str().unwrap().to_string();
            let dir = read_recursive(&path);
            node.size += dir.size();
            node.children.push(dir);
        }
    }

    node.children.sort_by(biggest_size_first);
    return FSNode::Directory(node);
}

fn biggest_size_first(lhs: &FSNode, rhs: &FSNode) -> Ordering {
    return lhs.size().cmp(&rhs.size()).reverse();
}
