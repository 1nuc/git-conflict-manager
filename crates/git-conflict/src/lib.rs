use std::{
    cell::RefMut,
    fs::remove_file,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use git2::{Commit, Error, Index, Oid, Repository, Signature, Status, StatusOptions, Time};

use crate::git_src::Branches;
pub mod combine;
pub mod discarding;
pub mod git_src;
pub mod merge_trees;
pub mod utils;

pub trait GitOps{
    fn call_discarding(&self, version: bool, overwrite: bool);
    fn call_combinition(&self, overwrite: bool);
    fn call_tree_merge(&self, version: bool, parent_interference: bool);
}

pub trait Initialize {
    fn init(branch_1: String, branch_2: String) -> Self;
    fn return_path() -> PathBuf;
    fn return_repo(file_path: PathBuf) -> Option<Repository>;
    fn does_conflict_exists(&self) -> bool;
}
pub trait Utils {
    fn find_ancesistor(&self, other_branch: &str) -> Result<Oid, Error>;
}

pub trait ManualControl: Actions {
    fn perform_manual_commit(&mut self, overwrite: bool) -> bool {
        let msg = format!(
            "Resolve Conflict: Merge {} branch into {} branch",
            self.branches().src_branch,
            self.branches().dest_branch
        );
        // get the heads commits
        // retreive the commits of "ours" branch and theres
        let ours_parents_commits = self
            .repo()
            .head()
            .expect("unable to find the head branch")
            .peel_to_commit()
            .expect("error peeling to commit in ours version")
            .id();
        let theirs_parents_commits = self
            .repo()
            .find_reference("MERGE_HEAD")
            .expect("unable to find the second theirs reference")
            .peel_to_commit()
            .expect("error in peeling to a commit in theirs version")
            .id();
        // retreive the commits of "theirs" branch

        //ancestor commit
        let ancestor = self
            .repo()
            .find_ancesistor(&self.branches().dest_branch)
            .expect("unable to find the ancestor oid");

        let parent_commits: &[Oid] = match overwrite {
            true => &[ancestor],
            false => &[ours_parents_commits, theirs_parents_commits],
        };
        self.commit(parent_commits, msg)
    }
}
/// A trait that contains the necessary action required by all methodologies to resolve a conflict
pub trait Actions {
    //Return the index
    fn branches(&self) -> Branches;

    fn index(&self) -> RefMut<'_,Index>;

    //Return the repo
    fn repo(&self) -> RefMut<'_,Repository>;

    //staging changes
    //this function has an embedding implementation
    fn staging(&mut self, files: Vec<String>) {
        let _ = files
            .iter()
            .map(|x| {
                let path = Path::new(x);
                self.index()
                    .add_path(path)
                    .expect("Error adding the file to the staging area");
            })
            .collect::<Vec<_>>();
        self.index()
            .write()
            .expect("unable to save the staged changes to memory");
    }

    //Making a commit
    //this function has an embedding implementation
    #[allow(unused_must_use)]
    fn commit(&mut self, parent_commits: &[Oid], msg: String) -> bool {
        let repo = self.repo();
        let tree = repo.find_tree(self.index().write_tree().unwrap()).unwrap();
        // Grabbing the details of the commit
        let signature = self.repo().signature().unwrap().to_owned();
        let config = self
            .repo()
            .config()
            .expect("Error in retreiving the configuration")
            .snapshot()
            .expect("error in creating a snapshot");
        // Considering the actual system time for establishing a commit
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        // Grabbing the author's details of the commit
        let user_name = config.get_str("user.name").unwrap();
        let email = config.get_str("user.email").unwrap();
        let time = Time::new(now, 0);
        let author = Signature::new(user_name, email, &time).expect("unable to create a borrow");
        let p_cm = parent_commits
            .iter()
            .map(|x| repo.find_commit(*x).unwrap())
            .collect::<Vec<Commit>>();
        let p_cm = p_cm.iter().collect::<Vec<&Commit>>();
        let p_commits = p_cm.as_slice();
        //rust git2 doesn't automatically clean up the conflict the conflict must be deleted

        match self
            .repo()
            .commit(Some("HEAD"), &author, &signature, &msg, &tree, p_commits)
        {
            Ok(_val) => {
                //after making the commit git must know that the commit is clearing the conflict
                //therefore, MERGE_HEAD file must be deleted to indicate the success of the merge
                let merge_head_path = self.repo().path().join("MERGE_HEAD"); // I believe this is the
                // line where the error
                // stems
                //repo.path outputs the content of the .git directory
                //join "MERGE_HEAD" finds the file that starts with MERGE HEAD
                if merge_head_path.exists() {
                    remove_file(merge_head_path).expect("unable to remove the file");
                    //deleting the merge conflict if the commit didn't auto delete
                }
                true
            }
            _ => false,
        }
    }

    fn does_conflict_exists(&self) -> bool {
        self.index().has_conflicts()
    }

    //return the file with conditions
    //this function has an embedding implementation
    fn return_conflicted_files(&self, condition: Status) -> Option<Vec<String>> {
        let repo = self.repo();
        let mut options = StatusOptions::new();
        options
            .include_untracked(false)
            .recurse_untracked_dirs(false);
        let status = repo.statuses(Some(&mut options)).unwrap();
        let mut list_of_conflicted_files = Vec::new();
        for i in status.iter() {
            if i.status().contains(condition)
                && let Some(path) = i.path()
            {
                list_of_conflicted_files.push(path.to_owned());
            }
        }
        Some(list_of_conflicted_files)
    }
}
