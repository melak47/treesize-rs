extern crate tabwriter;

use std::io::Write;
use self::tabwriter::TabWriter;

use super::tree::FSNode;

pub fn print_tree(tree: &FSNode, max_depth: i64, max_dir_entries: i64) {
    let mut tw = TabWriter::new(Vec::new());

    print_tree_impl(tree, &mut tw, "", 0, max_depth, max_dir_entries);

    tw.flush().unwrap();
    let bytes = tw.unwrap();
    let tabulated = String::from_utf8_lossy(&bytes);

	print!("{}", tabulated);
}

const SUM: &'static str = "(Σ)";
const BRANCH: &'static str = "├── ";
const LAST_BRANCH: &'static str = "└── ";
const INDENT: &'static str = "    ";
const NESTED_INDENT: &'static str = "│   ";

fn print_tree_impl<T: Write>(node: &FSNode, mut tw: &mut TabWriter<T>, prefix: &str, depth: i64, max_depth: i64, max_dir_entries: i64) {
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

    if max_depth < 0 || max_depth >= depth {
        let mut size_not_shown: u64 = 0;
        for (idx, item) in node.children().enumerate() {
            let last = idx == (node.children().count() - 1);
            let (branch, nested) = if last {
                (LAST_BRANCH, INDENT)
            } else {
                (BRANCH, NESTED_INDENT)
            };

            if max_dir_entries < 0 || (idx as i64) < max_dir_entries {
                write!(&mut tw, "{}{}", prefix, branch).unwrap();

                let nested_prefix = format!("{}{}", prefix, nested);
                print_tree_impl(item, &mut tw, &nested_prefix, depth + 1, max_depth, max_dir_entries);
            } else {
                size_not_shown += item.size();

                if last {
                    let _ = writeln!(&mut tw, "{}{}...\t{}\t{}", prefix, branch, human_readable_byte_size(size_not_shown), SUM);
                }
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
