use git2::{
    Commit, Error, Index, IndexConflict, IndexEntry, MergeOptions, Oid, build::CheckoutBuilder,
};
use std::{path::PathBuf};

use crate::{GitOps, Measuments, git_src::Repo};

impl<'a> Measuments<'a> for Repo<'a> {
    fn make_entry(
        &self,
        ancestor: &IndexEntry,
        base: &IndexEntry,
        parent_interference: bool,
    ) -> IndexEntry {
        let index = match parent_interference {
            true => ancestor,
            false => base,
        };

        IndexEntry {
            ctime: index.ctime,
            mtime: index.mtime,
            path: index.path.clone(),
            dev: base.dev,
            ino: base.ino,
            id: base.id,
            mode: base.mode,
            uid: base.uid,
            gid: base.gid,
            file_size: base.file_size,
            flags: index.path.len() as u16,
            flags_extended: index.flags_extended,
        }
    }
    fn apply_index_changes(&mut self, mut index: Index) {
        index
            .write()
            .expect("Error in writing the index back to the tree");
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
    fn resolve_conflict_tree_level(&self) -> (Index, Oid, Oid) {
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

        let merged_options = MergeOptions::default();
        // let mut checkout_builder=CheckoutBuilder::default();

        // The below trees are conflicted
        let mut merged_index = self
            .repo
            .merge_trees(
                &ancestor_tree,
                &src_branch_tree,
                &other_branch_tree,
                Some(&merged_options),
            )
            .unwrap();
        let conflicts = merged_index
            .conflicts()
            .unwrap()
            .collect::<Vec<Result<IndexConflict, _>>>();
        // the above index is created but its not connected to a repostiroy
        let index_path = self.repo.path().join("index");
        let mut index = Index::open(index_path.as_path()).expect("unable to create an index");
        index.clear().expect("unable to clear the index");
        let mut conflicted_files = Vec::new();
        conflicts
            .into_iter()
            .for_each(|conf: Result<IndexConflict, _>| {
                let entry = conf.unwrap();
                let ancestor = entry.ancestor.unwrap();
                let base = entry.our.unwrap();
                let theirs = entry.their.unwrap();
                index
                    .add(&self.make_entry(&ancestor, &base, true))
                    .expect("Error in resolving conflicted index entries");
                let conflicted_files_path = PathBuf::from(
                    String::from_utf8(theirs.path).expect("unable to get the file path"),
                );
                conflicted_files.push(conflicted_files_path);
            });
        // clearing the index from the conflicted files
        conflicted_files.into_iter().for_each(|f| {
            // delete the conflicts in the old index and add the remaining files to the updated index
            merged_index
                .conflict_remove(&f)
                .expect("unable to remove the entry");
        });

        // now adding the remaining entries to the index
        merged_index.iter().collect::<Vec<IndexEntry>>().into_iter().for_each(|x|{
            index
                .add(&x)
                .expect("error in adding the remaining entries");
        });

        (index, src_branch_commit.id(), ancestor.id())
    }

    fn print_index_contents(&self, index: &Index) {
        for entry in index.iter() {
            let path = String::from_utf8(entry.path).expect("unable to find path");
            if let Ok(obj) = self.repo.find_object(entry.id, None) {
                if let Some(blob) = obj.as_blob() {
                    let content = String::from_utf8(blob.content().to_vec())
                        .expect("unable to fetch the content");
                    println!("{:?}", path);
                    println!("{:?}", content);
                } else {
                    println!("No content to display");
                }
            } else {
                println!("No object with that entry");
            }
        }
    }
}
