use std::fs::Metadata;
#[cfg(target_os="linux")]
use std::os::unix::fs::MetadataExt;


pub enum FilesystemBehaviour {
    Traverse,
    OneFileSystemRoot,
    OneFileSystemChild(OneFileSystemParentInner),
}

impl FilesystemBehaviour {
    pub fn next_level(&self, going_into: &Metadata) -> Option<FilesystemBehaviour> {
        match self {
            FilesystemBehaviour::Traverse => Some(FilesystemBehaviour::Traverse),
            FilesystemBehaviour::OneFileSystemRoot => Some(FilesystemBehaviour::OneFileSystemChild(metadata_to_inner(going_into))),
            FilesystemBehaviour::OneFileSystemChild(parent) => {
                let this_inner = metadata_to_inner(going_into);
                if *parent == this_inner {
                    Some(FilesystemBehaviour::OneFileSystemChild(this_inner))
                } else {
                    None
                }
            }
        }
    }
}


#[cfg(target_os="linux")]
type OneFileSystemParentInner = u64;

/// This can be replaced with some variant of Option<u32> if/when std::os::windows::fs::MetadataExt::volume_serial_number() stabilises
/// https://github.com/rust-lang/rust/issues/63010
#[cfg(target_os="windows")]
type OneFileSystemParentInner = ();

#[cfg(not(any(target_os="linux", target_os="windows")))]
type OneFileSystemParentInner = ();

fn metadata_to_inner(meta: &Metadata) -> OneFileSystemParentInner {
    #[cfg(target_os="linux")]
    { meta.dev() }

    #[cfg(target_os="windows")]
    { drop(meta) }

    #[cfg(not(any(target_os="linux", target_os="windows")))]
    { drop(meta) }
}
