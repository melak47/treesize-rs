extern crate tabwriter;

use std::io::Write;
use self::tabwriter::TabWriter;

use super::tree::FSNode;



pub fn print_tree(tree: &FSNode) {
    let mut tw = TabWriter::new(Vec::new());
    let prefix = "".to_string();

    print_tree_impl(&tree, &mut tw, &prefix);

    tw.flush().unwrap();
    let tabulated = String::from_utf8(tw.unwrap()).unwrap();
    print!("{}", &tabulated);
}

pub fn print_tree_impl<T: Write>(node: &FSNode, mut tw: &mut TabWriter<T>, prefix: &String) {

    let sum_suffix = if node.is_dir() {
        "(Σ)"
    } else {
        ""
    };

    writeln!(&mut tw,
             "{}\t{}\t{}",
             node.name(),
             human_readable_byte_size(node.size()),
             &sum_suffix,
             )
        .unwrap();

    for (idx, item) in node.children().enumerate() {
        let last = idx == (node.children().count() - 1);
        let (branch, nested) = if last {
            ("└", "     ")
        } else {
            ("├", "│    ")
        };

        write!(&mut tw, "{}{}─── ", &prefix, &branch).unwrap();

        let new_prefix = concat(prefix, nested);
        print_tree_impl(&item, &mut tw, &new_prefix);
    }
}

fn concat(a: &str, b: &str) -> String {
    let mut result = a.to_string();
    result.push_str(b);
    return result;
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
