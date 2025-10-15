# rust-git-lite

A lightweight implementation of Git in Rust, built from scratch.
This project aims to replicate Git's core functionality from the ground up, focusing on correctness and clarity rather than full feature parity.

## 🛠️ Usage

Build and run the CLI:

```bash
cargo build --release
./target/release/rust-git-lite <command>
```

### Available Commands

#### Initialize a Repository

```bash
rust-git-lite init
```

#### Create a Blob Object

```bash
# Hash and write a file as a blob object
rust-git-lite hash-object -w hello.txt

# Just compute the hash without writing
rust-git-lite hash-object hello.txt
```

#### Read an Object

```bash
# Pretty-print the contents of an object
rust-git-lite cat-file -p <object-hash>
```

#### List Tree Contents

```bash
# Show full tree details
rust-git-lite ls-tree <tree-hash>

# Show only filenames
rust-git-lite ls-tree --name-only <tree-hash>
```

#### Create a Tree Object

```bash
# Create a tree from the current working directory
rust-git-lite write-tree
```

## ⚠️ Important Differences from Git

### `write-tree` Command

Unlike the actual `git write-tree` command, which creates a tree object from the current state of the **staging area** (where changes go when you run `git add`), this minimal implementation **does not implement a staging area**.

Instead, `write-tree` assumes that **all files in the working directory are staged**. This means it will create a tree object from the entire working directory, excluding the `.git` folder.
