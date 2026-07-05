use git2::{Index, Repository};

use crate::{
    Actions, Status,
    git_src::Repo,
};
use std::{
    cell::{RefMut},
    fs,
};

/// This struct specifies the methodology for merging both head and theirs versions
struct CmVersion<'a> {
    repo: Repo<'a>,
}

impl<'a> CmVersion<'a> {
    //resolves the conflict between two branches by combining the changes of both branches
    fn resolve_conflict_by_combining(&mut self) {
        let files = self.merge_files();
        self.staging(files); //stage the changes
        match self.perform_manual_commit() {
            //commit the changes
            true => println!("conflict is resolved"),
            false => panic!("error resolving the conflict"),
        }
    }

    fn remove_conflict_markers(&self, file_name: String) {
        let file_path = fs::read_to_string(&file_name).unwrap();
        let modify_content = file_path
            .lines()
            .filter(|x| !x.contains("<<<<<<<") & !x.contains("======") & !x.contains(">>>>>>"))
            .collect::<Vec<_>>()
            .join("\n");
        let _ = fs::write("tempfile", modify_content);
        let _ = fs::rename("tempfile", file_name);
        let _ = fs::remove_file("tempfile");
    }

    // This function merge the contents of the conflicted branches
    fn merge_files(&mut self) -> Vec<String> {
        // return the conflicted files
        let files = self
            .return_conflicted_files(Status::CONFLICTED)
            .expect("files cannot be found");
        // merge the contents of each file from the conflicted branches
        let _ = files
            .iter()
            .map(|x| {
                Self::remove_conflict_markers(self, x.to_string());
            })
            .collect::<Vec<_>>();
        files
    }

    fn perform_manual_commit(&mut self) -> bool {
        let msg = format!(
            "Resolve Conflict: Merge {} branch into {} branch",
            self.repo.branches.src_branch, self.repo.branches.dest_branch
        );
        // get the heads commits
        // retreive the commits of "ours" branch and theres
        let ours_parents_commits = self
            .repo
            .repo
            .0
            .borrow()
            .head()
            .expect("unable to find the head branch")
            .peel_to_commit()
            .expect("error peeling to commit in ours version")
            .id();
        let theirs_parents_commits = self
            .repo
            .repo
            .0
            .borrow()
            .find_reference("MERGE_HEAD")
            .expect("unable to find the second theirs reference")
            .peel_to_commit()
            .expect("error in peeling to a commit in theirs version")
            .id();
        // // retreive the commits of "theirs" branch
        let parent_commits = &[ours_parents_commits, theirs_parents_commits];
        self.commit(parent_commits, msg)
    }
}
impl<'a> Actions for CmVersion<'a> {
    fn index(&self) -> RefMut<Index> {
        self.repo.index.0.borrow_mut()
    }
    fn repo(&self) -> RefMut<Repository> {
        self.repo.repo.0.borrow_mut()
    }
}
