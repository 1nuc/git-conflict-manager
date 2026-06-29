use std::path::PathBuf;

use git2::{Commit, Error, Index, IndexEntry, Repository, Status};
pub mod git_src;
pub mod git_opt;

pub trait GitOps <'a>{
    fn find_ancesistor(&'a self)-> Result<Commit<'a>, Error>;
    fn merge_trees(&mut self);
    fn staging(&mut self, files: Vec<String>);
    fn commit(&mut self, mut index: Index, parent_commits: &[&Commit], msg: String)-> bool;
    fn return_conflicted_files(&self,condition: Status)-> Option<Vec<String>>;
    fn merge(&self,branch_1_commit: Commit, branch_2_commit: Commit) -> Result<Index, Error>;
    fn checkout_version(&mut self, ours: bool) -> &mut Self;
    fn checkout_files(&mut self) -> Vec<String>;
    fn resolve_conflict_by_discarding(&mut self);
    fn does_conflict_exists(&self) -> bool;
    fn remove_conflict_markers(&self, file_path: String);
    fn merge_files(&mut self) -> Vec<String>;
    fn resolve_conflict_by_combining(&mut self);
}
pub trait Initialize {
    fn init(branch_1: String, branch_2: String) -> Self;
    fn return_path() -> PathBuf;
    fn return_repo(file_path: PathBuf) -> Option<Repository>;
}
pub trait Measuments{
    fn make_entry(&self, ancestor:IndexEntry, base:IndexEntry, parent_interference: bool)-> IndexEntry;
    fn apply_index_changes(&mut self, index: Index);
}
