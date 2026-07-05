//!Define wrapper utils for efficient operation
//! This module defines the utility functions required to ensure the smoothness of conflict
//! resolution operation, for example the Index and Repository struct in lib git2 crate does not
//! implement the Copy and Clone traits. Those Traits are necessary to share the parameters across
//! different functions. Being able to mutually mutate the Index and Repository enables efficient
//! operation and avoid redundant code. Another major factor is the fact that those structs are
//! extensivly used, so the usage of Reference counter is necessary that provides mutliple versions
//! of one share instance in the memory, when combined with the ref cell, it allows for mutual
//! mutation.
use git2::{Error, Index, Oid, Repository, build::CheckoutBuilder};
use std::{cell::RefCell, rc::Rc};

use crate::Utils;

///wrapper for the Index of the lib git crate
pub struct NucIndex(pub Rc<RefCell<Index>>);

impl Clone for NucIndex {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

///wrapper for the Repository of the lib git crate
pub struct NucRepository(pub Rc<RefCell<Repository>>);

impl Clone for NucRepository {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl NucRepository {
    pub fn print_index_contents(&self, index: &Index) {
        for entry in index.iter() {
            let path = String::from_utf8(entry.path).expect("unable to find path");
            if let Ok(obj) = self.0.borrow().find_object(entry.id, None) {
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
///wrapper for the CheckoutBuilder of the lib git crate
pub struct NucCheckoutBuilder<'a>(pub Rc<RefCell<CheckoutBuilder<'a>>>);

impl<'a> Clone for NucCheckoutBuilder<'a> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}
impl Utils for Repository{

    /// find the ancestor commits and trees
    /// This function returns the ancestor OID that is required to fetch the commit and tree
    fn find_ancesistor(&self, other_branch: &str) -> Result<Oid, Error> {
        let head_commits = self.head().unwrap().peel_to_commit().unwrap();
        let other_branch_commits =self
            .find_branch(other_branch, git2::BranchType::Local)
            .unwrap()
            .into_reference()
            .peel_to_commit()
            .expect("unable to fetch the commit");
         self.merge_base(head_commits.id(), other_branch_commits.id())
    }

}
