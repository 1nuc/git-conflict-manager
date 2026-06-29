use std::sync::Arc;

use git2::{Index, IndexEntry, build::CheckoutBuilder};

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
    fn apply_index_changes(&mut self, mut index: Index){
        index.write().expect("Error in writing the tree");
        self.repo.set_index(&mut index).expect("Unable to write the index to the repository"); //staging
        let mut checkout_builder=CheckoutBuilder::new();
        self.repo.checkout_index(Some(&mut index), Some(checkout_builder.force())).unwrap();
        self.index=index;
    }
}

