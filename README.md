# treesize

[![crate.badge]][crate.link]
[![license.badge]][license.link]
[![commits.badge]][commits.link]

Linux: [![linux.build.badge]][linux.build.link]
Windows: [![windows.build.badge]][windows.build.link]

Commandline utility that lists a directory tree (like GNU tree) with file and directory sizes, sorted in descending order by size.

Example output:
```
> treesize
.                              15.5  KB  (Σ)
├── src                        6.5   KB  (Σ)
│   ├── directory              5.4   KB  (Σ)
│   │   ├── mod.rs             2.3   KB
│   │   ├── print.rs           1.9   KB
│   │   └── tree.rs            1.2   KB
│   └── main.rs                1.1   KB
├── appveyor_rust_install.ps1  2.7   KB
├── README.md                  2.2   KB
├── Cargo.lock                 1.6   KB
├── appveyor.yml               1.1   KB
├── LICENSE                    1.1   KB
└── Cargo.toml                 285   B
```



[crate.badge]: https://img.shields.io/crates/v/treesize.svg?maxAge=2592000?style=plastic
[crate.link]: https://crates.io/crates/treesize

[license.badge]: https://img.shields.io/crates/l/treesize.svg?maxAge=2592000?style=plastic
[license.link]: https://github.com/melak47/treesize-rs/blob/master/LICENSE

[commits.badge]: https://img.shields.io/github/commits-since/melak47/treesize-rs/v0.2.1.svg?maxAge=2592000?style=plastic
[commits.link]: https://github.com/melak47/treesize-rs

[linux.build.badge]: https://img.shields.io/travis/melak47/treesize-rs/master.svg?maxAge=2592000?style=plastic
[linux.build.link]: https://travis-ci.org/melak47/treesize-rs

[windows.build.badge]: https://ci.appveyor.com/api/projects/status/3as532ws1ib9re2x/branch/master?svg=true
[windows.build.link]: https://ci.appveyor.com/project/melak47/treesize-rs/branch/master
