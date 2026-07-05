use crate::{Actions, git_src::Repo};

struct TreeVersion<'a>{
    tr: Repo<'a>,
}

impl TreeVersion <'a>{

}

impl<'a> Actions for TreeVersion<'a>{
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
