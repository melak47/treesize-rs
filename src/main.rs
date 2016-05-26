#[macro_use]
extern crate clap;
use clap::Arg;
mod directory;

fn main() {
    let matches = clap::App::new(env!("CARGO_PKG_NAME"))
                      .version(concat!("v", env!("CARGO_PKG_VERSION")))
                      .about(env!("CARGO_PKG_DESCRIPTION"))
                      .author(env!("CARGO_PKG_AUTHORS"))
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

    match directory::read_recursive(std::path::Path::new(&path), ignore_dotfiles) {
        Ok(ref tree) => directory::print::print_tree(tree),
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    }
}
