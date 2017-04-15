use std::cmp::Ordering;
use std::{error, fmt, fs, io};
use std::path::{Path, PathBuf};
use std::io::Write;

pub mod tree;
pub mod print;

use self::tree::{DirectoryNode, FileNode, FSNode};

#[derive(Debug)]
pub enum ReadDirError {
    IoError {
        err: io::Error,
        path: PathBuf,
    },
}

impl fmt::Display for ReadDirError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ReadDirError::IoError { ref err, ref path } => {
                write!(f,
                       "Error reading {:?}, caused by I/O error: {}",
                       path,
                       err)
            }
        }
    }
}

impl error::Error for ReadDirError {
    fn description(&self) -> &str {
        match *self {
            ReadDirError::IoError { ref err, .. } => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ReadDirError::IoError { ref err, .. } => Some(err),
        }
    }
}

macro_rules! try_path {
    ($expr:expr, $path:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => {
                return Err(ReadDirError::IoError{err: err, path: $path.to_path_buf()});
            }
        }
    }
}

pub fn read_recursive(path: &Path, ignore_dotfiles: bool, follow_symlinks: bool) -> Result<FSNode, ReadDirError> {
    let name = path.file_name().unwrap_or(path.as_os_str()).to_string_lossy().to_string();
    let mut node = DirectoryNode::new(name);

    for entry in try_path!(fs::read_dir(&path), path) {
        let entry = try_path!(entry, path);
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        if ignore_dotfiles && name.starts_with('.') {
            continue;
        }

        let meta = if follow_symlinks {
            // does traverse symlinks
            try_path!(fs::metadata(&path), path)
        } else {
            // does not traverse symlinks, like fs::symlink_metadata
            try_path!(entry.metadata(), &path)
        };

        if meta.is_file() {
            node.children.push(FSNode::File(FileNode {
                name: name,
                size: meta.len(),
            }));
            node.size += meta.len();
        } else if meta.is_dir() {
            match read_recursive(&path, ignore_dotfiles, follow_symlinks) {
                Ok(dir) => {
                    node.size += dir.size();
                    node.children.push(dir);
                }
                Err(err) => try_path!(writeln!(io::stderr(), "{}", err), Path::new("stderr")),
            }
        }
    }

    node.children.sort_by(biggest_size_first);
    Ok(FSNode::Directory(node))
}

fn biggest_size_first(lhs: &FSNode, rhs: &FSNode) -> Ordering {
    lhs.size().cmp(&rhs.size()).reverse()
}
