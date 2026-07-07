use std::cell::RefMut;

use git2::{Index, Repository, Status};

use crate::{
    Actions, ManualControl,
    git_src::{Branches, Repo},
};

#[derive(Clone)]
pub struct DsVersion<'a> {
    ds: Repo<'a>,
}

impl<'a> DsVersion<'a> {
    pub fn new(repo: Repo<'a>) -> Self{
        Self{
            ds: repo,
        }
    }
    // The checkout is the first step towards changing the index
    pub fn checkout_version(&mut self, ours: bool) -> &mut Self {
        let head_branch = self
            .ds
            .repo
            .0
            .borrow()
            .head()
            .expect("unable to return the reference")
            .shorthand()
            .expect("unable to retrieve the branch namepointed by the head")
            .to_string();
        //checking the branch pointed by the head to build the checkout
        if head_branch != self.ds.branches.src_branch && head_branch != self.ds.branches.dest_branch
        {
            panic!("head is not pointing to any branch");
        } else {
            match ours {
                true => self.ds.builder.0.borrow_mut().use_ours(true),
                false => self.ds.builder.0.borrow_mut().use_theirs(true),
            };
        }
        self
    }

    //this function has an embedding implementation
    #[allow(unused_must_use)]
    pub fn checkout_files(&mut self) -> Vec<String> {
        //add files paths to be checked out with the new merge
        let files = self
            .return_conflicted_files(Status::CONFLICTED)
            .expect("files cannot be found");
        // specify the files for which the checkout is to be held for
        files
            .iter()
            .map(|x| {
                //the below function adds the files to the checkout builder
                self.ds.builder.0.borrow_mut().path(x).force();
            })
            .collect::<Vec<_>>();
        files
    }

    //resolves the conflict between two branches by discarding the changes of either two branches
    pub fn resolve_conflict_by_discarding(&mut self, overwrite: bool, version: bool) {
        let files = self.checkout_files();
        let self_cloned = self.clone(); //must copy as mut
        let mut index = self_cloned.index();
        let mut builder = self_cloned.ds.builder.0.borrow_mut();
        // dereferencing the value to get the inner contents
        let _ = self
            .ds
            .repo
            .0
            .borrow_mut()
            .checkout_index(Some(&mut *index), Some(&mut *builder)); //revert back the index to match the index to the checkout builder
        drop(index);
        self.staging(files); //stage the changes
        match self.perform_manual_commit(overwrite, version) {
            //commit the changes
            true => println!("conflict is resolved"),
            false => panic!("error resolving the conflict"),
        }
    }
}
impl<'a> Actions for DsVersion<'a> {
    fn index(&self) -> RefMut<'_,Index> {
        self.ds.index.0.borrow_mut()
    }

    fn repo(&self) -> RefMut<'_,Repository> {
        self.ds.repo.0.borrow_mut()
    }

    fn branches(&self) -> Branches {
        self.ds.branches.clone()
    }
}
impl<'a> ManualControl for DsVersion<'a> {}
