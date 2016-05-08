#[macro_use]
extern crate clap;
use clap::Arg;
mod directory;

fn main() {
    // TODO: replace strings duplicated from Cargo.toml once 1.9.0 hits stable
    //                           env!("CARGO_PKG_NAME")
    let matches = clap::App::new("treesize")
                      .version(env!("CARGO_PKG_VERSION"))
    //                       env!("CARGO_PKG_DESCRIPTION")
                      .about("Print directory tree (like GNU tree), sorted by size")
    //                        env!("CARGO_PKG_AUTHORS")
                      .author("melak47 <melak47@gmail.com>")
                      .arg(Arg::with_name("DIRECTORY")
                               .help("Directory to list")
                               .index(1)
                               .default_value("."))
                      .arg(Arg::with_name("all")
                               .help("List all files (including dotfiles)")
                               .short("a"))
                      .get_matches();

    let ignore_dotfiles = matches.occurrences_of("all") == 0;
    let path = matches.value_of("DIRECTORY").unwrap().to_string();

    let tree = directory::read_recursive(&path, ignore_dotfiles);
    directory::print::print_tree(&tree);
}
