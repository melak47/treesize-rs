use std::cmp::Ordering;
use std::fs;
use std::path::Path;

pub mod tree;
pub mod print;
use self::tree::{DirectoryNode, FileNode, FSNode};

pub fn read_recursive(path: &String) -> DirectoryNode {
    let path = fs::canonicalize(Path::new(&path)).unwrap();
    let mut node = DirectoryNode::new(path.file_name().unwrap().to_str().unwrap().to_owned());

    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let meta = entry.metadata().unwrap();
        let name = entry.file_name().to_str().unwrap().to_owned();

        if meta.is_file() {
            node.children.push(FSNode::File(FileNode {
                name: name,
                size: meta.len(),
            }));
            node.size += meta.len();
        } else if meta.is_dir() {
            let path = entry.path().to_str().unwrap().to_owned();
            let dir = read_recursive(&path);
            node.size += dir.size;
            node.children.push(FSNode::Directory(dir));
        }
    }

    node.children.sort_by(biggest_size_first);
    return node;
}

fn biggest_size_first(lhs: &FSNode, rhs: &FSNode) -> Ordering {
    let lhs_size = match lhs {
        &FSNode::Directory(ref d) => d.size,
        &FSNode::File(ref f) => f.size,
    };
    let rhs_size = match rhs {
        &FSNode::Directory(ref d) => d.size,
        &FSNode::File(ref f) => f.size,
    };
    return rhs_size.cmp(&lhs_size);
}
