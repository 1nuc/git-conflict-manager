# git_conflict

[![Crates.io](https://img.shields.io/crates/v/git_conflict.svg)](https://crates.io/crates/git_conflict)
[![docs.rs](https://docs.rs/git_conflict/badge.svg)](https://docs.rs/git_conflict)
[![License](https://img.shields.io/crates/l/git_conflict.svg)](../../LICENSE)

Core library for git conflict detection and resolution. Provides traits and implementations for staging, committing, checking out, and resolving conflicts between branches using the `git2` crate.

---

## Features

- Detect whether conflicts exist in the current index
- Check out local or foreign branch versions of conflicted files
- Strip conflict markers and combine changes from both branches
- Stage and commit resolved files programmatically

---

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
git_conflict = "0.1.2"
```

Initialize a `Repo` and resolve conflicts:

```rust

use git_conflict::{git_operations::Repo, GitOps, Initialize};

let mut repo = Repo::init("main".to_string(), "feature".to_string());

if repo.does_conflict_exists() {
    repo.checkout_local().resolve_conflict_by_discarding();
}
```

---

## Traits

**`Initialize`** — construct a `Repo` from the current working directory.

**`GitOps`** — conflict resolution operations:
- `does_conflict_exists` — check if the index has conflicts
- `checkout_local` / `checkout_foreign` — select which side to keep
- `resolve_conflict_by_discarding` — keep one side, discard the other
- `resolve_conflict_by_combining` — strip markers and keep both sides
- `staging` / `commit` — stage and commit resolved files

---

## License

MIT — see [LICENSE](../../LICENSE)
