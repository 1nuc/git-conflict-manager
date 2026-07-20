# Git Conflict Manager Using Rust 

[![Test](https://github.com/1nuc/git-conflict-manager/actions/workflows/automate_build.yml/badge.svg)](https://github.com/1nuc/git-conflict-manager/actions/workflows/automate_build.yml)
[![CI](https://github.com/1nuc/git-conflict-manager/actions/workflows/automate_build.yml/badge.svg)](https://github.com/1nuc/git-conflict-manager/actions/workflows/automate_build.yml)
[![Crates.io](https://img.shields.io/crates/v/git_conflict.svg)](https://crates.io/crates/git_conflict)
[![docs.rs](https://docs.rs/git_conflict/badge.svg)](https://docs.rs/git_conflict)
[![License](https://img.shields.io/crates/l/git_conflict.svg)](LICENSE)

<img width="800" height="509" alt="simulation" src="https://github.com/user-attachments/assets/a4ca0287-5699-4a07-87c1-e234d8ac1441" />

A Rust library and CLI tool for automating git conflict resolution. Define custom strategies and rules to resolve conflicts automatically, without manual intervention.

Git conflicts are an inevitable part of collaborative development, yet resolving them remains a tedious and error-prone manual process. **Git Conflict Manager** eliminates that friction by providing a structured, automated approach to conflict resolution — letting you choose from a set of well-defined strategies and execute them in a single command.

---

## Workspace Structure

This repository is organized as a Cargo workspace containing two crates:

| Crate | Description |
|-------|-------------|
| [`crates/git-conflict`](./crates/git-conflict) | Core library — conflict detection, resolution strategies, and git operations |
| [`crates/git-conflict-cli`](./crates/git-conflict-cli) | CLI tool — interactive terminal interface built on top of the library |

---

## Installation

### Via Cargo (Recommended)

If you have Rust and Cargo installed, this is the simplest way:

```bash
cargo install git-conflict-cli
```

This builds the binary in release mode and places it in `~/.cargo/bin`, which is automatically on your `PATH`. You can then run `git-conflict-cli` from anywhere.

### From Source (via Makefile)

Clone the repository and use the provided `Makefile` to build and install system-wide:

```bash
git clone git@github.com:1nuc/git-conflict-manager.git
cd git-conflict-manager
make install
```

This compiles a release build and copies the binary to `/usr/local/bin/gcm`, making it available system-wide without needing Cargo in your `PATH`.

To uninstall:

```bash
make uninstall
```

---

## Usage

Navigate to a repository that has a conflict, then run:

```bash
gcm  
```

- `src_branch` — the branch currently pointed to by `HEAD` (your local branch)
- `dest_branch` — the branch you are merging into

**Example:**

```bash
gcm main feature/my-feature
```

You will be presented with an interactive menu:

```
Git Conflict Manager.... The tool for ultimate file control

Which conflict resolution would you like to choose:
Option 1: Keep Local Head Changes
Option 2: Keep Foreign Branch Changes
Option 3: Remove Markers and Keep Both Changes
```

Select a strategy by entering its number. The tool will automatically stage and commit the resolved files.

> **Tip:** To check which branch `HEAD` is pointing to, run `git status` before invoking the tool.

---

## Resolution Strategies

| Option | Strategy | Description |
|--------|----------|-------------|
| 1 | Keep Local | Discards incoming changes and keeps your current branch's version |
| 2 | Keep Foreign | Discards local changes and keeps the incoming branch's version |
| 3 | Combine Both | Strips conflict markers and retains content from both branches |

---

## Crates on crates.io

- **Library:** [crates.io/crates/git_conflict](https://crates.io/crates/git_conflict)
- **CLI:** [crates.io/crates/git-conflict-cli](https://crates.io/crates/git-conflict-cli)

To use the library in your own project, add this to your `Cargo.toml`:

```toml
[dependencies]
git_conflict = "0.1.2"
```

---

## License

This project is licensed under the MIT License. See [LICENSE](./LICENSE) for details.
