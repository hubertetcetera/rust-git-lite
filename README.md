> [!WARNING]
> This project is still **under development**. At the moment it can only read and create blob objects. Check back as more functionality is added!

# rust-git-lite

A lightweight implementation of Git in Rust, built from scratch.
This project aims to replicate Git’s core functionality from the ground up, focusing on correctness and clarity rather than full feature parity.

## 🚧 Status

Implemented:

- ✅ Initialize the `.git` directory (`init`)
- ✅ Read a blob object (`cat-file`)
- ✅ Create a blob object (`hash-object`)

Work in progress:

- ⏳ Read a tree object (`ls-tree`)
- ⏳ Write a tree object (`write-tree`)
- ⏳ Create a commit (`commit-tree`)
- ⏳ Clone a repository (`clone`)

---

## 🛠️ Usage

Build and run the CLI:

```bash
cargo build --release
./target/release/rust-git-lite <command>
```

Examples:

```bash
# Initialize a new repository
rust-git-lite init

# Write a file as a blob object
rust-git-lite hash-object -w hello.txt
```

## 📂 Project Goal

The end goal is a fully working minimal Git, capable of:

- Initializing the `.git` directory
- Reading and creating objects (blobs, trees)
- Creating commits
- Cloning a repository
