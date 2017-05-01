# treesize

[![crate.badge]][crate.link]
[![license.badge]][license.link]
[![commits.badge]][commits.link]

Linux: [![linux.build.badge]][linux.build.link]
Windows: [![windows.build.badge]][windows.build.link]

`treesize` is a small command-line utility for listing a directory tree sorted by size. It's main goal is to quickly provide an overview of what is taking up the most space in a tree, so by default it will only list the 5 largest entries in the specified directory (and summarize the rest):

```
> treesize
.                              201.5  MB  (Σ)
├── target                     201.5  MB  (Σ)
├── src                        8.6    KB  (Σ)
├── Cargo.lock                 4.7    KB
├── appveyor_rust_install.ps1  2.7    KB
├── README.md                  2.1    KB
└── ...                        2.4    KB  (Σ)
```

You can use these options to tweak the output listing (traversal of the tree is not affected):

```
-d <max-depth>          Maximal directory depth to recurse, or -1 for infinite [default: 0]
-e <max-entries>        Maximum number of entries to display per directory, or -1 for infinite [default: 5]
```

For example: 

```
> treesize -d2 -e3
.                         201.5  MB  (Σ)
├── target                201.5  MB  (Σ)
│   ├── debug             137.5  MB  (Σ)
│   │   ├── deps          131.6  MB  (Σ)
│   │   ├── treesize.exe  3.1    MB
│   │   ├── build         2.8    MB  (Σ)
│   │   └── ...           260    B   (Σ)
│   └── release           64.0   MB  (Σ)
│       ├── deps          60.7   MB  (Σ)
│       ├── build         2.5    MB  (Σ)
│       ├── treesize.exe  800.0  KB
│       └── ...           262    B   (Σ)
├── src                   8.6    KB  (Σ)
│   ├── directory         6.1    KB  (Σ)
│   │   ├── print.rs      2.5    KB
│   │   ├── mod.rs        2.4    KB
│   │   └── tree.rs       1.2    KB
│   └── main.rs           2.5    KB
├── Cargo.lock            4.7    KB
└── ...                   7.2    KB  (Σ)
```

Dot-files and symlinks are ignored unless you use these flags:

```
-a    List all files (including dotfiles)
-L    Follow any symbolic links encountered
```

[crate.badge]: https://img.shields.io/crates/v/treesize.svg?maxAge=2592000?style=plastic
[crate.link]: https://crates.io/crates/treesize

[license.badge]: https://img.shields.io/crates/l/treesize.svg?maxAge=2592000?style=plastic
[license.link]: https://github.com/melak47/treesize-rs/blob/master/LICENSE

[commits.badge]: https://img.shields.io/github/commits-since/melak47/treesize-rs/v0.3.0.svg?maxAge=2592000?style=plastic
[commits.link]: https://github.com/melak47/treesize-rs

[linux.build.badge]: https://img.shields.io/travis/melak47/treesize-rs/master.svg?maxAge=2592000?style=plastic
[linux.build.link]: https://travis-ci.org/melak47/treesize-rs

[windows.build.badge]: https://ci.appveyor.com/api/projects/status/3as532ws1ib9re2x/branch/master?svg=true
[windows.build.link]: https://ci.appveyor.com/project/melak47/treesize-rs/branch/master
