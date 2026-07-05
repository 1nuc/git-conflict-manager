use crate::{GitOps, Initialize, Measuments, utils::{NucCheckoutBuilder, NucIndex, NucRepository}};
use git2::{
    Commit, Index, Oid, Repository, Signature, Status, StatusOptions, Time, build::CheckoutBuilder,
};
use std::{
    cell::RefCell, env, fs, path::{Path, PathBuf}, rc::Rc, time::{SystemTime, UNIX_EPOCH}
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
}

#[allow(non_snake_case)]
impl<'a> GitOps<'a> for Repo<'a> {

    /// If you want to have the cimmits of both branches run this function
    #[allow(unused_must_use)]
    #[allow(unused_variables)]
    fn merge_trees(&mut self) {
        let (index, src_commit, ancestor) = self.resolve_conflict_tree_level();

        let msg = format!(
            "Resolve Conflict through tree resolution:  {} branch into {} branch",
            self.branches.src_branch, self.branches.dest_branch
        );
        // get the heads commits
        let parent_commits = &[src_commit, ancestor];

        // Apply the index changes to the repository
        self.apply_index_changes(index);

        // TODO: Fix the commit call function 
        // match self.repo.0.borrow().commit(parent_commits, msg) {
        //     true => println!("conflict is resolved"),
        //     false => panic!("error resolving the conflict"),
        // }
    }

    // The checkout is the first step towards changing the index
    fn checkout_version(&mut self, ours: bool) -> &mut Self {
        let head_branch = self
            .repo.0.borrow()
            .head()
            .expect("unable to return the reference")
            .shorthand()
            .expect("unable to retrieve the branch namepointed by the head")
            .to_string();
        //checking the branch pointed by the head to build the checkout
        if head_branch != self.branches.src_branch && head_branch != self.branches.dest_branch {
            panic!("head is not pointing to any branch");
        } else {
            match ours {
                true => self.builder.0.borrow_mut().use_ours(true),
                false => self.builder.0.borrow_mut().use_theirs(true),
            };
        }
        self
    }

    //this function has an embedding implementation
    #[allow(unused_must_use)]
    fn checkout_files(&mut self) -> Vec<String> {
        //add files paths to be checked out with the new merge
        let files = self
            .return_conflicted_files(Status::CONFLICTED)
            .expect("files cannot be found");
        // specify the files for which the checkout is to be held for
        files
            .iter()
            .map(|x| {
                //the below function adds the files to the checkout builder
                self.builder.path(x).force();
            })
            .collect::<Vec<_>>();
        files
    }

    //resolves the conflict between two branches by discarding the changes of either two branches
    fn resolve_conflict_by_discarding(&mut self) {
        let files = self.checkout_files();
        let _ = self
            .repo
            .checkout_index(Some(&mut self.index), Some(&mut self.builder)); //revert back the index to match the index to the checkout builder
        self.staging(files); //stage the changes
        match self.perform_manual_commit() {
            //commit the changes
            true => println!("conflict is resolved"),
            false => panic!("error resolving the conflict"),
        }
    }

}
