use git2::{Commit, Error, Index, Status};
mod git_operations;

pub trait GitOps{
    fn staging(&mut self, files: Vec<String>);
    fn commit(&mut self)-> bool;
    fn return_files(&self,condition: Status)-> Option<Vec<String>>;
    fn merge(&self,branch_1_commit: Commit, branch_2_commit: Commit) -> Result<Index, Error>;
}
