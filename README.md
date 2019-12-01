# treesize

[![crate.badge]][crate.link]
[![license.badge]][license.link]
[![commits.badge]][commits.link]

CI status: [![build.badge]][build.link]

`treesize` is a small command-line utility for listing a directory tree sorted by size. It's main goal is to quickly provide an overview of what is taking up the most space in a tree, so by default it will only list the 5 largest entries in the specified directory (and summarize the rest):

```
> treesize
.               99.4  MiB  (Σ)
├── target      99.4  MiB  (Σ)
├── src         13.4  KiB  (Σ)
├── Cargo.lock  4.9   KiB
├── README.md   3.2   KiB
├── LICENSE     1.1   KiB
└── ...         271   B    (Σ)
```

You can use these options to tweak the output listing (traversal of the tree is not affected):

```
-d <max-depth>          Maximal directory depth to recurse, or -1 for infinite [default: 0]
-e <max-entries>        Maximum number of entries to display per directory, or -1 for infinite [default: 5]
-s <size-format>        How to format node sizes: h/human – powers of 1024, H/si – powers of 1000, r/raw – no
                        folding [default: human]
```

For example:

```
> treesize -d2 -e3
.                          99.4   MiB  (Σ)
├── target                 99.4   MiB  (Σ)
│   ├── debug              75.4   MiB  (Σ)
│   │   ├── deps           53.4   MiB  (Σ)
│   │   ├── treesize.pdb   10.1   MiB
│   │   ├── build          5.0    MiB  (Σ)
│   │   └── ...            6.9    MiB  (Σ)
│   └── release            24.0   MiB  (Σ)
│       ├── deps           18.0   MiB  (Σ)
│       ├── build          3.6    MiB  (Σ)
│       ├── treesize.pdb   1.6    MiB
│       └── ...            756.7  KiB  (Σ)
├── src                    13.4   KiB  (Σ)
│   ├── directory          9.4    KiB  (Σ)
│   │   ├── print.rs       4.0    KiB
│   │   ├── mod.rs         2.6    KiB
│   │   ├── filesystem.rs  1.6    KiB
│   │   └── ...            1.2    KiB  (Σ)
│   └── main.rs            4.0    KiB
├── Cargo.lock             4.9    KiB
└── ...                    4.5    KiB  (Σ)
```

Flags that affect the traversal include:

```
-a                   List all files (including dotfiles)
-L                   Follow any symbolic links encountered
--one-file-system    Stay in the same file system when listing
```

[crate.badge]: https://img.shields.io/crates/v/treesize.svg?maxAge=2592000?style=plastic
[crate.link]: https://crates.io/crates/treesize

[license.badge]: https://img.shields.io/crates/l/treesize.svg?maxAge=2592000?style=plastic
[license.link]: https://github.com/melak47/treesize-rs/blob/master/LICENSE

[commits.badge]: https://img.shields.io/github/commits-since/melak47/treesize-rs/v0.5.0.svg?maxAge=2592000?style=plastic
[commits.link]: https://github.com/melak47/treesize-rs

[build.badge]: https://img.shields.io/travis/melak47/treesize-rs/master.svg?maxAge=2592000?style=plastic
[build.link]: https://travis-ci.org/melak47/treesize-rs
