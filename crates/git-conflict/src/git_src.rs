use crate::{Initialize, utils::{NucCheckoutBuilder, NucIndex, NucRepository}};
use git2::{
    Error, Oid, Repository, build::CheckoutBuilder
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

    /// find the ancestor commits and trees
    /// This function returns the ancestor OID that is required to fetch the commit and tree
    fn find_ancesistor(&self, other_branch: &str) -> Result<Oid, Error> {
        let repo=self.repo.0.borrow();
        let head_commits = repo.head().unwrap().peel_to_commit().unwrap();
        let other_branch_commits =repo
            .find_branch(other_branch, git2::BranchType::Local)
            .unwrap()
            .into_reference()
            .peel_to_commit()
            .expect("unable to fetch the commit");
         repo.merge_base(head_commits.id(), other_branch_commits.id())
    }

}

