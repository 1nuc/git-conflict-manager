use std::path::PathBuf;

use git2::{Commit, Error, Index, Repository, Status};
pub mod git_operations;

pub trait GitOps <'a>{
    fn staging(&mut self, files: Vec<String>);
    fn commit(&mut self)-> bool;
    fn return_files(&self,condition: Status)-> Option<Vec<String>>;
    fn merge(&self,branch_1_commit: Commit, branch_2_commit: Commit) -> Result<Index, Error>;
    fn checkout_local(&mut self);
    fn checkout_foreign(&mut self);
    fn checkout_files(&mut self) -> Vec<String>;
    fn resolve_conflict_by_discarding(&mut self);
    fn does_conflict_exists(&self) -> bool;
}
pub trait Initialize {
    fn init(branch_1: String, branch_2: String) -> Self;
    fn return_path() -> PathBuf;
    fn return_repo(file_path: PathBuf) -> Option<Repository>;
}
