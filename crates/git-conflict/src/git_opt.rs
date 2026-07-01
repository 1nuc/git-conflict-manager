use git2::{Commit, Error, Index, IndexEntry, MergeOptions, build::CheckoutBuilder};
use std::{path::PathBuf, sync::Arc};

use crate::{GitOps, Measuments, git_src::Repo};

impl<'a> Measuments<'a> for Repo<'a> {
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
        // get the heads commits
        // retreive the commits of "ours" branch and theres
        let ours_parents_commits = self
            .repo
            .head()
            .expect("unable to find the head branch")
            .peel_to_commit()
            .expect("error peeling to commit in ours version")
            .id();
        let theirs_parents_commits = self
            .repo
            .find_reference("MERGE_HEAD")
            .expect("unable to find the second theirs reference")
            .peel_to_commit()
            .expect("error in peeling to a commit in theirs version")
            .id();
        // // retreive the commits of "theirs" branch
        let parent_commits = &[ours_parents_commits, theirs_parents_commits];
        self.commit(parent_commits, msg)
    }

    /// find the ancestor commits and trees
    fn find_ancesistor(&'a self) -> Result<Commit<'a>, Error> {
        let head_commits = self.repo.head().unwrap().peel_to_commit().unwrap();
        let other_branch_commits = self
            .repo
            .find_branch(&self.branches.dest_branch, git2::BranchType::Local)
            .unwrap()
            .into_reference()
            .peel_to_commit()
            .expect("unable to fetch the commit");
        let oid = self
            .repo
            .merge_base(head_commits.id(), other_branch_commits.id())
            .unwrap();
        self.repo.find_commit(oid)
    }

    #[allow(unused_must_use)]
    fn resolve_conflict_tree_level(&'a self) -> (Index, Commit<'a>, Commit<'a>) {
        let src_branch = self.repo.head().expect("unable to get the head");

        let src_branch_commit = src_branch
            .peel_to_commit()
            .expect("unable to fetch the commit");
        let src_branch_tree = src_branch_commit.tree().expect("unable to fetch the tree");

        let other_branch = self
            .repo
            .find_branch(&self.branches.dest_branch, git2::BranchType::Local)
            .expect("unable to fetch other branch")
            .into_reference();

        let other_branch_tree = other_branch
            .peel_to_commit()
            .expect("unable to fetch the commit in the dest branch")
            .tree()
            .expect("unable to fetch the tree in the dest branch");

        let ancestor = self
            .find_ancesistor()
            .expect("There is no common parent between those commits");

        let ancestor_tree = ancestor.tree().unwrap();

        let mut merged_options = MergeOptions::default();
        // let mut checkout_builder=CheckoutBuilder::default();

        // The below trees are conflicted
        let mut merged_index = self
            .repo
            .merge_trees(
                &ancestor_tree,
                &src_branch_tree,
                &other_branch_tree,
                Some(merged_options.patience(true)),
            )
            .unwrap();
        let conflicts = merged_index.conflicts().unwrap();
        // the above index is created but its not connected to a repostiroy
        let mut index = Index::new().unwrap();
        let mut conflicted_files=Vec::new();
        conflicts.map(|conf| {
            let entry = conf.unwrap();
            let ancestor = entry.ancestor.unwrap();
            let their = entry.their.unwrap();
            let base = entry.our.unwrap();
            index.add(&self.make_entry(ancestor, base, true)).expect("Error in resolving conflicted index entries");
            let conflicted_files_path =PathBuf::from(String::from_utf8(their.path).expect("unable to get the file path"));
            conflicted_files.push(conflicted_files_path);
        });
        // clearing the index from the conflicted files
        conflicted_files.into_iter().for_each(|f|{
        // delete the conflicts in the old index and add the remaining files to the updated index
            merged_index.conflict_remove(&f).expect("unable to remove the entry");
        });

        // now adding the remaining entries to the index
        merged_index.iter().map(|x|{
            index.add(&x).expect("error in adding the remaining entries");
        });

        (index, src_branch_commit, ancestor)
    }
}
