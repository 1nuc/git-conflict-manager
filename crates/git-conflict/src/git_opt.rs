use git2::{Index, IndexEntry, build::CheckoutBuilder};
use std::sync::Arc;

use crate::{GitOps, Measuments, git_src::Repo};

impl<'a> Measuments for Repo<'a> {
    fn make_entry(
        &self,
        ancestor: IndexEntry,
        base: IndexEntry,
        parent_interference: bool,
    ) -> IndexEntry {
        let base_copy = Arc::new(base);
        let index = match parent_interference {
            true => ancestor,
            false => Arc::into_inner(base_copy.clone()).unwrap(),
        };
        IndexEntry {
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
    fn apply_index_changes(&mut self, mut index: Index) {
        index.write().expect("Error in writing the tree");
        self.repo
            .set_index(&mut index)
            .expect("Unable to write the index to the repository"); //staging
        let mut checkout_builder = CheckoutBuilder::new();
        self.repo
            .checkout_index(Some(&mut index), Some(checkout_builder.force()))
            .unwrap();
        self.index = index;
    }
    fn perform_manual_commit(&mut self) -> bool {
        let msg = format!(
            "Resolve Conflict: Merge {} branch into {} branch",
            self.branches.src_branch, self.branches.dest_branch
        );
        let object = Arc::new(&mut *self);
        let mut_self = object.clone();
        // get the heads commits
        let head = mut_self.repo.head().unwrap();
        // // retreive the commits of "ours" branch
        let ours_parents_commits = head
            .peel_to_commit()
            .expect("error peeling to commit in ours version");
        let theirs = mut_self
            .repo
            .find_reference("MERGE_HEAD")
            .expect("unable to find the second theirs reference");
        // // retreive the commits of "theirs" branch
        let theirs_parents_commits = theirs
            .peel_to_commit()
            .expect("error peeling to a commit in theirs version");
        let parent_commits = &[&ours_parents_commits, &theirs_parents_commits];
        Arc::into_inner(object).unwrap().commit(parent_commits, msg)
    }
}
