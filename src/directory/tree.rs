use std::iter;

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

impl FSNode {
    pub fn children<'a>(&'a self) -> Box<iter::Iterator<Item = &'a FSNode> + 'a> {
        match self {
            &FSNode::Directory(ref d) => Box::new(d.children.iter()),
            &FSNode::File(ref _f) => Box::new(iter::empty()),
        }
    }
    pub fn name(&self) -> &String {
        match self {
            &FSNode::Directory(ref d) => &d.name,
            &FSNode::File(ref f) => &f.name,
        }
    }

    pub fn size(&self) -> u64 {
        match self {
            &FSNode::Directory(ref d) => d.size,
            &FSNode::File(ref f) => f.size,
        }
    }

    pub fn is_dir(&self) -> bool {
        match self {
            &FSNode::Directory(ref _d) => true,
            &FSNode::File(ref _f) => false,
        }
    }
}
