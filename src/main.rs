extern crate clap;
use clap::Arg;
use std::str::FromStr;
mod directory;


fn validate_int(s: String) -> Result<(), String> {
    match i64::from_str(&s) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:?} is not a valid integer: {}", s, e))
    }
}

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
                            .arg(Arg::with_name("follow-symlinks")
                                     .help("Follow any symbolic links encountered")
                                     .short("L"))
                            .arg(Arg::with_name("max-depth")
                                     .help("Maximal directory depth to recurse, or -1 for infinite")
                                     .short("d")
                                     .default_value("0")
                                     .takes_value(true)
                                     .validator(validate_int))
                            .arg(Arg::with_name("max-entries")
                                     .help("Maximum number of entries to display per directory, or -1 for infinite")
                                     .short("e")
                                     .default_value("5")
                                     .takes_value(true)
                                     .validator(validate_int))
                            .get_matches();

    let ignore_dotfiles = !matches.is_present("all");
    let follow_symlinks = matches.is_present("follow-symlinks");
    let max_depth = i64::from_str(matches.value_of("max-depth").unwrap()).unwrap();
    let max_entries = i64::from_str(matches.value_of("max-entries").unwrap()).unwrap();
    let path = matches.value_of("DIRECTORY").unwrap();

    directory::print::print_tree(&directory::read_recursive(std::path::Path::new(&path), ignore_dotfiles, follow_symlinks), max_depth, max_entries);
}
