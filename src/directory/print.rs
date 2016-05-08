extern crate tabwriter;

use std::io::Write;
use self::tabwriter::TabWriter;

use super::tree::FSNode;

pub fn print_tree(tree: &FSNode) {
    let mut tw = TabWriter::new(Vec::new());

    print_tree_impl(&tree, &mut tw, "");

    tw.flush().unwrap();
    let bytes = tw.unwrap();
    let tabulated = String::from_utf8_lossy(&bytes);

    // avoid https://github.com/rust-lang/rust/issues/23344
    // by wriring smaller chunks
    // TODO: remove once 1.9 hits stable
    for line in tabulated.split("\n") {
        println!("{}", line);
    }
}
const SUM: &'static str = "(Σ)";
const BRANCH: &'static str = "├── ";
const LAST_BRANCH: &'static str = "└── ";
const INDENT: &'static str = "    ";
const NESTED_INDENT: &'static str = "│   ";

fn print_tree_impl<T: Write>(node: &FSNode, mut tw: &mut TabWriter<T>, prefix: &str) {

    let sum_suffix = if node.is_dir() {
        SUM
    } else {
        ""
    };

    writeln!(&mut tw,
             "{}\t{}\t{}",
             node.name(),
             human_readable_byte_size(node.size()),
             sum_suffix,
             )
        .unwrap();

    for (idx, item) in node.children().enumerate() {
        let last = idx == (node.children().count() - 1);
        let (branch, nested) = if last {
            (LAST_BRANCH, INDENT)
        } else {
            (BRANCH, NESTED_INDENT)
        };

        write!(&mut tw, "{}{}", prefix, branch).unwrap();

        let nested_prefix = format!("{}{}", prefix, nested);
        print_tree_impl(&item, &mut tw, &nested_prefix);
    }
}

fn human_readable_byte_size(bytes: u64) -> String {
    if bytes < 1024 {
        return format!("{}\tB", bytes);
    }

    let metric_prefix = ["B", "KB", "MB", "GB", "TB", "PB", "EB"];

    let which = log2(bytes) / 10;
    let decimal = bytes as f64 / 1024.0_f64.powf(which as f64);
    return format!("{:.1}\t{}", decimal, metric_prefix[which as usize]);
}

fn log2(mut x: u64) -> u64 {
    let mut n: u64 = 0;

    while (x >> 1) > 0 {
        x >>= 1;
        n += 1;
    }
    n
}
