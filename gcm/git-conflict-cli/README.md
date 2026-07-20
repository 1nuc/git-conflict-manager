# git-conflict-cli


<img width="800" height="418" alt="sim" src="https://github.com/user-attachments/assets/709aeb6b-76d4-44f5-92ea-6a03964b365f" />


[![Crates.io](https://img.shields.io/crates/v/git-conflict-cli.svg)](https://crates.io/crates/git-conflict-cli)
[![License](https://img.shields.io/crates/l/git-conflict-cli.svg)](../../LICENSE)

Interactive TUI tool for resolving git conflicts. Built on top of the [`git_conflict`](../git-conflict) library.

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
| 4 | Merge the selected branch's tree with ancestor |

---

## License

MIT — see [LICENSE](../../LICENSE)
