use crate::{GitOps, Initialize, Measuments};
use git2::{
    Commit, Config, Index, Oid, Repository, Signature, Status, StatusOptions, Time,
    build::CheckoutBuilder,
};
use std::{
    env, fs,
    path::{Path, PathBuf},
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

//define the base struct to obtain the branches naming
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
pub struct Repo<'a> {
    pub path: PathBuf,
    pub repo: Repository,
    pub index: Index, //this is the where the index of the files getting updated
    pub branches: Branches,
    pub builder: CheckoutBuilder<'a>,
}

#[allow(non_snake_case)]
impl<'a> Initialize for Repo<'a> {
    //init
    fn init(branch_1: String, branch_2: String) -> Self {
        let file_path = Self::return_path();
        let Repo = Self::return_repo(file_path).expect("unable to find a git repository");
        let repo_path = Repo
            .workdir()
            .expect("unable to find the repository path")
            .to_path_buf();
        let Index = Repo.index().expect("unable to find the index");
        //prepare the details needed to perform git operations
        Self {
            path: repo_path,
            repo: Repo,
            index: Index,
            branches: Branches::init(branch_1, branch_2),
            builder: CheckoutBuilder::new(),
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
    //staging changes
    //this function has an embedding implementation
    fn staging(&mut self, files: Vec<String>) {
        let _ = files
            .iter()
            .map(|x| {
                let path = Path::new(x);
                self.index
                    .add_path(path)
                    .expect("Error adding the file to the staging area");
            })
            .collect::<Vec<_>>();
        self.index
            .write()
            .expect("unable to save the staged changes to memory");
    }

    /// If you want to have the cimmits of both branches run this function
    #[allow(unused_must_use)]
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

        match
           self
            .commit(parent_commits, msg)
        {
            true => println!("conflict is resolved"),
            false => panic!("error resolving the conflict"),
        }
    }
    //Making a commit
    //this function has an embedding implementation
    #[allow(unused_must_use)]
    fn commit(&mut self, parent_commits: &[Oid], msg: String) -> bool {
        let tree = self
            .repo
            .find_tree(self.index.write_tree().unwrap())
            .unwrap();
        // Grabbing the details of the commit
        let signature = self.repo.signature().unwrap().to_owned();
        let config = Config::new().unwrap();
        // Considering the actual system time for establishing a commit
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        // Grabbing the author's details of the commit
        let user_name = config.get_str("user.name").unwrap();
        let email = config.get_str("user.email").unwrap();
        let time = Time::new(now, 0);
        let author = Signature::new(user_name, email, &time).expect("unable to create a borrow");
        let p_cm = parent_commits
            .iter()
            .map(|x| self.repo.find_commit(*x).unwrap())
            .collect::<Vec<Commit>>();
        let p_cm = p_cm.iter().collect::<Vec<&Commit>>();
        let p_commits = p_cm.as_slice();
        //rust git2 doesn't automatically clean up the conflict the conflict must be deleted

        match self
            .repo
            .commit(Some("HEAD"), &author, &signature, &msg, &tree, p_commits)
        {
            Ok(_val) => {
                //after making the commit git must know that the commit is clearing the conflict
                //therefore, MERGE_HEAD file must be deleted to indicate the success of the merge
                let merge_head_path = self.repo.path().join("MERGE_HEAD"); // I believe this is the
                // line where the error
                // stems
                //repo.path outputs the content of the .git directory
                //join "MERGE_HEAD" finds the file that starts with MERGE HEAD
                if merge_head_path.exists() {
                    fs::remove_file(merge_head_path).expect("unable to remove the file");
                    //deleting the merge conflict if the commit didn't auto delete
                }
                true
            }
            _Error => false,
        }
    }

    //return the file with conditions
    //this function has an embedding implementation
    fn return_conflicted_files(&self, condition: Status) -> Option<Vec<String>> {
        let mut options = StatusOptions::new();
        options
            .include_untracked(false)
            .recurse_untracked_dirs(false);
        let status = self.repo.statuses(Some(&mut options)).unwrap();
        let mut list_of_conflicted_files = Vec::new();
        for i in status.iter() {
            if i.status().contains(condition)
                && let Some(path) = i.path()
            {
                list_of_conflicted_files.push(path.to_owned());
            }
        }
        Some(list_of_conflicted_files)
    }

    // The checkout is the first step towards changing the index
    fn checkout_version(&mut self, ours: bool) -> &mut Self {
        let head_branch = self
            .repo
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
                true => self.builder.use_ours(true),
                false => self.builder.use_theirs(true),
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

    fn does_conflict_exists(&self) -> bool {
        self.index.has_conflicts()
    }
    //resolve conflict by merging the changes from both branches : e.g. delete the conflict
    //markers

    //resolves the conflict between two branches by combining the changes of both branches
    fn resolve_conflict_by_combining(&mut self) {
        let files = self.merge_files();
        self.staging(files); //stage the changes
        match self.perform_manual_commit() {
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
