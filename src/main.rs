mod directory;

fn main() {
    let path = std::env::args().nth(1).unwrap_or(".".to_owned());
    let tree = directory::read_recursive(&path);

    directory::print::print_tree(&tree);
}
