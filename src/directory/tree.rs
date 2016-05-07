pub struct DirectoryNode {
    pub name: String,
    pub children: Vec<FSNode>,
    pub size: u64,
}

impl DirectoryNode {
    pub fn new(name: String) -> DirectoryNode {
        DirectoryNode {
            name: name,
            children: Vec::new(),
            size: 0,
        }
    }
}

pub struct FileNode {
    pub name: String,
    pub size: u64,
}

pub enum FSNode {
    Directory(DirectoryNode),
    File(FileNode),
}
