use git2::{Index, Repository};

use crate::{
    Actions, ManualControl, Status,
    git_src::{Branches, Repo},
};
use std::{cell::RefMut, fs};

/// This struct specifies the methodology for merging both head and theirs versions
pub struct CmVersion<'a> {
    cm: Repo<'a>,
}

impl<'a> CmVersion<'a> {

    pub fn new(repo: Repo<'a>) -> Self{
        Self{
            cm: repo,
        }
    }
    pub fn resolve_conflict_by_combining(&mut self) {
            let files = self.merge_files();
            self.staging(files); //stage the changes
            match self.perform_manual_commit(false) {
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
}
impl<'a> Actions for CmVersion<'a> {
    fn index(&self) -> RefMut<'_,Index> {
        self.cm.index.0.borrow_mut()
    }
    fn repo(&self) -> RefMut<'_,Repository> {
        self.cm.repo.0.borrow_mut()
    }
    fn branches(&self) -> Branches {
        self.cm.branches.clone()
    }
}
impl<'a> ManualControl for CmVersion<'a> {}
