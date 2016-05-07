extern crate tabwriter;

use std::io::Write;
use std::cmp::Ordering;
use tabwriter::TabWriter;

struct DirectoryNode {
    name: String,
    children: Vec<FSNode>,
    size: u64,
}

impl DirectoryNode {
    fn new(name: String) -> DirectoryNode {
        DirectoryNode {
            name: name,
            children: Vec::new(),
            size: 0,
        }
    }
}

struct FileNode {
    name: String,
    size: u64,
}

enum FSNode {
    Directory(DirectoryNode),
    File(FileNode),
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



fn human_readable_byte_size(bytes: u64) -> String {

    fn log2(mut x: u64) -> u64 {
        let mut n: u64 = 0;
        while (x >> 1) > 0 {
            x >>= 1;
            n += 1;
        }
        return n;
    }

    if bytes < 1024 {
        return format!("{}\tB", bytes);
    }

    let metric_prefix = ["B", "KB", "MB", "GB", "TB", "PB", "EB"];

    let which = log2(bytes) / 10;
    let decimal = bytes as f64 / 1024.0_f64.powf(which as f64);
    return format!("{:.1}\t{}", &decimal, &metric_prefix[which as usize]);
}

fn print_tree_impl<T: Write>(tree: &DirectoryNode, mut tw: &mut TabWriter<T>, prefix: &String) {

    for (idx, entry) in (&tree.children).into_iter().enumerate() {
        let branch = if idx == tree.children.len() - 1 {
            '└'
        } else {
            '├'
        };

        match entry {
            &FSNode::File(ref f) => {
                writeln!(&mut tw,
                         "{}{}── {}\t{}\t",
                         &prefix,
                         &branch,
                         &f.name,
                         human_readable_byte_size(f.size))
                    .unwrap();
            }
            &FSNode::Directory(ref d) => {

                writeln!(&mut tw,
                         "{}{}── {}\t{}\t(Σ)",
                         &prefix,
                         &branch,
                         &d.name,
                         human_readable_byte_size(d.size))
                    .unwrap();

                let mut new_prefix = prefix.clone();
                if idx < tree.children.len() - 1 {
                    new_prefix.push_str("│   ");
                } else {
                    new_prefix.push_str("    ");
                }
                print_tree_impl(&d, &mut tw, &new_prefix);
            }
        }
    }
}

fn print_tree(tree: &DirectoryNode) {
    let mut tw = TabWriter::new(Vec::new());

    writeln!(&mut tw,
             "{}\t{}\t(Σ)",
             &tree.name,
             human_readable_byte_size(tree.size))
        .unwrap();

    let prefix = "".to_owned();
    print_tree_impl(&tree, &mut tw, &prefix);

    tw.flush().unwrap();
    let tabulated = String::from_utf8(tw.unwrap()).unwrap();
    print!("{}", tabulated);
}

fn read_dir_recursive(path: &String) -> DirectoryNode {
    let path = std::fs::canonicalize(std::path::Path::new(&path)).unwrap();
    let mut node = DirectoryNode::new(path.file_name().unwrap().to_str().unwrap().to_owned());

    for entry in std::fs::read_dir(path).unwrap() {
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
            let dir = read_dir_recursive(&path);
            node.size += dir.size;
            node.children.push(FSNode::Directory(dir));
        }
    }

    node.children.sort_by(biggest_size_first);
    return node;
}

fn main() {
    let path = std::env::args().nth(1).unwrap_or(".".to_owned());
    let tree = read_dir_recursive(&path);

    print_tree(&tree);
}
