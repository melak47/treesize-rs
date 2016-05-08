#[macro_use]
extern crate clap;
use clap::Arg;
mod directory;

fn main() {
    let matches = clap::App::new(env!("CARGO_PKG_NAME"))
                      .version(env!("CARGO_PKG_VERSION"))
                      .author(env!("CARGO_PKG_AUTHORS"))
                      .about(env!("CARGO_PKG_DESCRIPTION"))
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
