use std::sync::Arc;

use git2::{Index, IndexEntry};

use crate::{git_src::Repo, Measuments};

impl <'a> Measuments for Repo<'a>{
    fn make_entry(&self, ancestor:IndexEntry, base:IndexEntry, parent_interference: bool) -> IndexEntry{
        let base_copy=Arc::new(base);
        let index=match parent_interference{
            true => ancestor,
            false=> Arc::into_inner(base_copy.clone()).unwrap(),
        };
        IndexEntry{
            ctime: index.ctime,
            mtime: index.mtime,
            path: index.path,
            dev: base_copy.dev,
            ino: base_copy.ino,
            id: base_copy.id,
            mode: base_copy.mode,
            uid: base_copy.uid,
            gid: base_copy.gid,
            file_size: base_copy.file_size,
            flags: index.flags,
            flags_extended: index.flags_extended,
        }
    }
}

