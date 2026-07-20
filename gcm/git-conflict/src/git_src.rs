use crate::{GitOps, Initialize, combine::CmVersion, discarding::DsVersion, merge_trees::TreeVersion, utils::{NucCheckoutBuilder, NucIndex, NucRepository}};
use git2::{
    Repository, build::CheckoutBuilder
};
use std::{
    cell::RefCell, env, path::{PathBuf}, rc::Rc,
};

//define the base struct to obtain the branches naming
#[derive(Clone)]
pub struct Branches {
    pub src_branch: String,
    pub dest_branch: String,
}

impl Branches {
    fn init(branch_1: String, branch_2: String) -> Self {
        Branches {
            src_branch: branch_1.to_string(),
            dest_branch: branch_2.to_string(),
        }
    }
}
//creating a struct that contains the essential details for a branch
#[allow(dead_code)]
#[derive(Clone)]
pub struct Repo<'a> {
    pub path: PathBuf,
    pub repo: NucRepository,
    pub index: NucIndex, //this is the where the index of the files getting updated
    pub branches: Branches,
    pub builder: NucCheckoutBuilder<'a>,
}

#[allow(non_snake_case)]
impl<'a> Initialize for Repo<'a> {
    //init
    fn init(branch_1: String, branch_2: String) -> Self {
        let file_path = Self::return_path();
        let repo = NucRepository(Rc::new(RefCell::new(Self::return_repo(file_path).expect("unable to find a git repository"))));
        // let Repo = Self::return_repo(file_path).expect("unable to find a git repository");
        let path= repo.0.borrow()
            .workdir()
            .expect("unable to find the repository path")
            .to_path_buf();
        let index = NucIndex(Rc::new(RefCell::new(repo.0.borrow().index().expect("unable to find the index"))));
        //prepare the details needed to perform git operations
        let builder= NucCheckoutBuilder(Rc::new(RefCell::new(CheckoutBuilder::new())));

        let branches= Branches::init(branch_1, branch_2);

        Self {
            path,
            repo,
            index,
            branches,
            builder,

        }
    }

    //TODO: return the directory as an environment variable
    fn return_path() -> PathBuf {
        env::current_dir().unwrap()
    }
    //Returning the directory path
    fn return_repo(file_path: PathBuf) -> Option<Repository> {
        // recursively traversing the directory to find the git index for which the content of the
        // .git folder exists
        match Repository::discover(file_path) {
            Ok(repo) => {
                if repo.workdir().is_some() {
                    Some(repo)
                } else {
                    panic!("no path found for this repo");
                }
            }
            _Error => {
                panic!("Unable to find the repository path");
            }
        }
    }

    fn does_conflict_exists(&self) -> bool {
        self.index.0.borrow().has_conflicts()
    }
}

impl <'a>GitOps for Repo<'a>{
    /// This function will call the discarding methodology
    /// version is the type of changes should we accept
    /// if true then the changes of the head will be accepted
    /// if false then the changes of the incoming branch will be accepted
    /// overwrite is to specify whether to ignore or write the conflicted commits of both branches
    fn call_discarding(&self, version: bool) {
        let mut object=DsVersion::new(self.clone());
        object.checkout_version(version).resolve_conflict_by_discarding(true);
    }

    /// This method call the combination approach
    /// overwrite bool is the same as before specify whether to keep or ignore previous commits 
    fn call_combinition(&self) {
        let mut object=CmVersion::new(self.clone());
        object.resolve_conflict_by_combining();
    }

    /// This is the fancy approach
    /// apply the merge in the index, resolve the conflict in the tree
    /// The tree and the index gets cleaned before even getting moved to the commit stage
    /// parent interference, specifies whether the parent previous commit should be combined with
    /// the new tree.
    /// if set to false then the approach output resembles the discarding method
    /// version is to specify ours or theirs
    fn call_tree_merge(&self, version: bool, parent_interference: bool) {

        let mut object=TreeVersion::new(self.clone(), version, parent_interference);
        object.merge_trees();
    }
}

