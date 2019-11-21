use std::cmp::Ordering;
use std::path::Path;
use std::io::Write;
use std::{fs, io};

pub mod tree;
pub mod print;
pub mod filesystem;

use self::tree::{DirectoryNode, FileNode, FSNode};
use self::filesystem::FilesystemBehaviour;

pub fn read_recursive(path: &Path, ignore_dotfiles: bool, follow_symlinks: bool, fs_behav: FilesystemBehaviour) -> FSNode {
    let name = path.file_name().unwrap_or(path.as_os_str()).to_string_lossy().to_string();
    let mut node = DirectoryNode::new(name);

    match fs::read_dir(&path) {
        Ok(entries) => {
            for entry in entries {
                let entry = match entry {
                    Ok(e) => e,
                    Err(err) => {
                        let _ = writeln!(io::stderr(), "Error reading {:?}, caused by I/O error: {}", path, err);
                        continue;
                    }
                };
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();

                if ignore_dotfiles && name.starts_with('.') {
                    continue;
                }

                let meta = match if follow_symlinks {
                    // does traverse symlinks
                    fs::metadata(&path)
                } else {
                    // does not traverse symlinks, like fs::symlink_metadata
                    entry.metadata()
                } {
                    Ok(m) => m,
                    Err(err) => {
                        let _ = writeln!(io::stderr(), "Error reading {:?}, caused by I/O error: {}", path, err);
                        continue;
                    }
                };

                if meta.is_file() {
                    node.children.push(FSNode::File(FileNode {
                        name: name,
                        size: meta.len(),
                    }));
                    node.size += meta.len();
                } else if meta.is_dir() {
                    if let Some(new_fs_behav) = fs_behav.next_level(&meta) {
                        let dir = read_recursive(&path, ignore_dotfiles, follow_symlinks, new_fs_behav);
                        node.size += dir.size();
                        node.children.push(dir);
                    }
                }
            }
        }
        Err(err) => {
            let _ = writeln!(io::stderr(), "Error reading {:?}, caused by I/O error: {}", path, err);
        }
    }

    node.children.sort_by(biggest_size_first);
    FSNode::Directory(node)
}

fn biggest_size_first(lhs: &FSNode, rhs: &FSNode) -> Ordering {
    lhs.size().cmp(&rhs.size()).reverse()
}
