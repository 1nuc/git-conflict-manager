use git2::{Commit, Error, Index, Status};
mod git_operations;

pub trait GitOps <'a>{
    fn staging(&mut self, files: Vec<String>);
    fn commit(&mut self)-> bool;
    fn return_files(&self,condition: Status)-> Option<Vec<String>>;
    fn merge(&self,branch_1_commit: Commit, branch_2_commit: Commit) -> Result<Index, Error>;
    fn checkout_local(&mut self);
    fn checkout_foreign(&mut self);
    fn checkout_files(&mut self) -> Vec<String>;
    fn resolve_conflict_by_discarding(&mut self);
}
