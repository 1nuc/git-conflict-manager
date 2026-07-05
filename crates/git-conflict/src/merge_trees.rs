use crate::{Actions, Utils, git_src::Repo};
use git2::{Index, IndexConflict, IndexEntry, MergeOptions, Oid, build::CheckoutBuilder};
use std::path::PathBuf;

pub struct TreeVersion<'a> {
    tr: Repo<'a>,
}

impl<'a> TreeVersion<'a> {
    pub fn merge_trees(&mut self) {
        let (index, src_commit, ancestor) = self.resolve_conflict_tree_level();

        let msg = format!(
            "Resolve Conflict through tree resolution:  {} branch into {} branch",
            self.tr.branches.src_branch, self.tr.branches.dest_branch
        );
        // get the heads commits
        let parent_commits = &[src_commit, ancestor];

        // Apply the index changes to the repository
        self.apply_index_changes(index);

        // TODO: Fix the commit call function
        match self.commit(parent_commits, msg) {
            true => println!("conflict is resolved"),
            false => panic!("error resolving the conflict"),
        }
    }

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
        self.tr
            .repo
            .0
            .borrow_mut()
            .set_index(&mut index)
            .expect("Unable to write the index to the repository"); //staging
        let mut checkout_builder = CheckoutBuilder::new();
        self.tr
            .repo
            .0
            .borrow_mut()
            .checkout_index(Some(&mut index), Some(checkout_builder.force()))
            .unwrap();
        *self.tr.index.0.borrow_mut() = index;
    }

    #[allow(unused_must_use)]
    fn resolve_conflict_tree_level(&self) -> (Index, Oid, Oid) {
        let (mut merged_index, src_branch_commit, ancestor) = self.merging_index();
        let repo = self.tr.repo.0.borrow();
        let conflicts = merged_index
            .conflicts()
            .unwrap()
            .collect::<Vec<Result<IndexConflict, _>>>();
        // the above index is created but its not connected to a repostiroy
        let index_path = repo.path().join("index");
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
        merged_index
            .iter()
            .collect::<Vec<IndexEntry>>()
            .into_iter()
            .for_each(|x| {
                index
                    .add(&x)
                    .expect("error in adding the remaining entries");
            });

        (index, src_branch_commit, ancestor)
    }

    fn merging_index(&self) -> (Index, Oid, Oid) {
        let repo = self.tr.repo.0.borrow();
        let src_branch = repo.head().expect("unable to get the head");

        let src_branch_commit = src_branch
            .peel_to_commit()
            .expect("unable to fetch the commit");
        let src_branch_tree = src_branch_commit.tree().expect("unable to fetch the tree");

        let other_branch = repo
            .find_branch(&self.tr.branches.dest_branch, git2::BranchType::Local)
            .expect("unable to fetch other branch")
            .into_reference();

        let other_branch_tree = other_branch
            .peel_to_commit()
            .expect("unable to fetch the commit in the dest branch")
            .tree()
            .expect("unable to fetch the tree in the dest branch");

        let ancestor = repo
            .find_commit(
                repo.find_ancesistor(&self.tr.branches.dest_branch)
                    .expect("There is no common parent between those commits"),
            )
            .expect("Unable to find the commit");

        let ancestor_tree = ancestor.tree().unwrap();

        let merged_options = MergeOptions::default();
        // let mut checkout_builder=CheckoutBuilder::default();

        // The below trees are conflicted
        let merged_index = repo
            .merge_trees(
                &ancestor_tree,
                &src_branch_tree,
                &other_branch_tree,
                Some(&merged_options),
            )
            .expect("Error in merging the index");

        (merged_index, src_branch_commit.id(), ancestor.id())
    }
}

impl<'a> Actions for TreeVersion<'a> {
    fn index(&self) -> std::cell::RefMut<git2::Index> {
        self.tr.index.0.borrow_mut()
    }

    fn repo(&self) -> std::cell::RefMut<git2::Repository> {
        self.tr.repo.0.borrow_mut()
    }

    fn branches(&self) -> crate::git_src::Branches {
        self.tr.branches.clone()
    }
}
