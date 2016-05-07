extern crate tabwriter;

use std::io::Write;
use self::tabwriter::TabWriter;

use directory::tree::{DirectoryNode, FSNode};

pub fn print_tree(tree: &DirectoryNode) {
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

fn human_readable_byte_size(bytes: u64) -> String {


    if bytes < 1024 {
        return format!("{}\tB", bytes);
    }

    let metric_prefix = ["B", "KB", "MB", "GB", "TB", "PB", "EB"];

    let which = log2(bytes) / 10;
    let decimal = bytes as f64 / 1024.0_f64.powf(which as f64);
    return format!("{:.1}\t{}", &decimal, &metric_prefix[which as usize]);
}

fn log2(mut x: u64) -> u64 {
    let mut n: u64 = 0;

    while (x >> 1) > 0 {
        x >>= 1;
        n += 1;
    }
    return n;
}
