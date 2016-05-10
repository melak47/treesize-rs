mod directory;

fn main() {
    let path = std::env::args().nth(1).unwrap_or(".".to_string());

    match directory::read_recursive(std::path::Path::new(&path)) {
        Ok(ref tree) => directory::print::print_tree(&tree),
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    }
}
