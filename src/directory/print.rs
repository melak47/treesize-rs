extern crate tabwriter;

use std::fmt;
use std::io::Write;
use std::str::FromStr;
use self::tabwriter::TabWriter;

use super::tree::FSNode;

pub fn print_tree(tree: &FSNode, max_depth: i64, max_dir_entries: i64, size_format: SizeFormat) {
    let mut tw = TabWriter::new(Vec::new());

    print_tree_impl(tree, &mut tw, "", 0, max_depth, max_dir_entries, size_format);

    tw.flush().unwrap();
    let bytes = tw.into_inner().unwrap();
    let tabulated = String::from_utf8_lossy(&bytes);

	print!("{}", tabulated);
}

const SUM: &'static str = "(Σ)";
const BRANCH: &'static str = "├── ";
const LAST_BRANCH: &'static str = "└── ";
const INDENT: &'static str = "    ";
const NESTED_INDENT: &'static str = "│   ";

fn print_tree_impl<T: Write>(node: &FSNode, mut tw: &mut TabWriter<T>, prefix: &str, depth: i64, max_depth: i64, max_dir_entries: i64, size_format: SizeFormat) {
    let sum_suffix = if node.is_dir() {
        SUM
    } else {
        ""
    };

    writeln!(&mut tw,
             "{}\t{}\t{}",
             node.name(),
             size_format.human_readable_byte_size(node.size()),
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
                print_tree_impl(item, &mut tw, &nested_prefix, depth + 1, max_depth, max_dir_entries, size_format);
            } else {
                size_not_shown += item.size();

                if last {
                    let _ = writeln!(&mut tw, "{}{}...\t{}\t{}", prefix, branch, size_format.human_readable_byte_size(size_not_shown), SUM);
                }
            }

        }
    }
}

#[derive(Copy, Clone)]
pub enum SizeFormat {
    Human,
    Si,
    Raw,
}

impl FromStr for SizeFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<SizeFormat, ()> {
        match s {
            "h" | "human" => Ok(SizeFormat::Human),
            "H" | "si" => Ok(SizeFormat::Si),
            "r" | "raw" => Ok(SizeFormat::Raw),
            _ => Err(()),
        }
    }
}

impl SizeFormat {
    pub const VALUES: &'static [&'static str] = &["h", "human", "H", "si", "r", "raw"];

    pub fn human_readable_byte_size(self, bytes: u64) -> SizeFormatter {
        SizeFormatter(self, bytes)
    }
}

pub struct SizeFormatter(SizeFormat, u64);

impl fmt::Display for SizeFormatter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        static HUMAN_PREFIX: &[&str] = &["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"];
        static SI_PREFIX: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB", "EB"];

        let (power, prefix) = match self.0 {
            SizeFormat::Human => (1024, HUMAN_PREFIX),
            SizeFormat::Si => (1000, SI_PREFIX),
            SizeFormat::Raw => return self.fmt_raw(f),
        };

        if self.1 < power {
            return self.fmt_raw(f);
        }

        let which = log2(self.1) / 10;
        let decimal = self.1 as f64 / (power as f64).powf(which as f64);
        write!(f, "{:.1}\t{}", decimal, prefix[which as usize])
    }
}

impl SizeFormatter {
    fn fmt_raw(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\tB", self.1)
    }
}

fn log2(mut x: u64) -> u64 {
    let mut n: u64 = 0;

    while (x >> 1) > 0 {
        x >>= 1;
        n += 1;
    }
    n
}
