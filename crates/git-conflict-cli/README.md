# git-conflict-cli


https://github.com/user-attachments/assets/0ea9fdf0-fb10-4621-8217-c1988a17c8ec


[![Crates.io](https://img.shields.io/crates/v/git-conflict-cli.svg)](https://crates.io/crates/git-conflict-cli)
[![License](https://img.shields.io/crates/l/git-conflict-cli.svg)](../../LICENSE)

Interactive CLI tool for resolving git conflicts. Built on top of the [`git_conflict`](../git-conflict) library.

---

## Installation

```bash
cargo install git-conflict-cli
```

---

## Usage

From inside a git repository that has a conflict:

```bash
gcm <src_branch> <dest_branch>
```

**Example:**

```bash
gcm main feature/my-feature
```

---

## Options

| Option | Description |
|--------|-------------|
| 1 | Keep local HEAD changes |
| 2 | Keep foreign branch changes |
| 3 | Remove markers and keep both changes |

---

## License

MIT — see [LICENSE](../../LICENSE)
