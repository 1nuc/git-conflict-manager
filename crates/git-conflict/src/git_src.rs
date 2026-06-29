use git2::{Commit, Error, Index, MergeOptions, Repository, Status, StatusOptions, build::CheckoutBuilder};
use crate::{GitOps, Initialize, Measuments};
use std::{env, fs, path::{Path, PathBuf}};

//define the base struct to obtain the branches naming
pub struct Branches{
    src_branch: String,
    dest_branch: String,
}

impl Branches{
   fn init(branch_1: String, branch_2: String) -> Self{
       Branches{
           src_branch: branch_1.to_string(),
           dest_branch: branch_2.to_string(),
       }
   } 
}
//creating a struct that contains the essential details for a branch 
#[allow(dead_code)]
pub struct Repo<'a>{
    pub path: PathBuf,
    pub repo: Repository,
    pub index: Index, //this is the where the index of the files getting updated
    pub branches: Branches,
    pub builder: CheckoutBuilder<'a>,
}

#[allow(non_snake_case)]
impl <'a> Initialize for Repo<'a>{
    //init
    fn init(branch_1: String, branch_2: String) -> Self{
        let file_path=Self::return_path();
        let Repo=Self::return_repo(file_path).expect("unable to find a git repository");
        let repo_path=Repo.workdir().expect("unable to find the repository path").to_path_buf();
        let Index=Repo.index().expect("unable to find the index");
        //prepare the details needed to perform git operations
        Self{
            path: repo_path,
            repo: Repo,
            index: Index,
            branches: Branches::init(branch_1, branch_2),
            builder: CheckoutBuilder::new(),
        }
    }

    //TODO: return the directory as an environment variable
    fn return_path() -> PathBuf{
        env::current_dir().unwrap()
    }
    //Returning the directory path
    fn return_repo(file_path: PathBuf) -> Option<Repository>{
       // recursively traversing the directory to find the git index for which the content of the
       // .git folder exists
       match Repository::discover(file_path){
           Ok(repo) => {
               if repo.workdir().is_some(){
                   Some(repo)
               }
               else{
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
impl <'a>GitOps<'a> for Repo<'a>{

    //staging changes
    //this function has an embedding implementation
    fn staging(&mut self, files: Vec<String>){
        let _=files.iter().map(|x| {
            let path=Path::new(x);
            self.index.add_path(path).expect("Error adding the file to the staging area");
            }).collect::<Vec<_>>();
        self.index.write().expect("unable to save the staged changes to memory");
    }

    /// find the ancestor commits and trees
    fn find_ancesistor(&'a self)-> Result<Commit<'a>, Error>{
        let head_commits=self.repo.head().unwrap().peel_to_commit().unwrap();
        let other_branch_commits=self.repo.
            find_branch(&self.branches.dest_branch,
            git2::BranchType::Local).unwrap().into_reference().peel_to_commit().expect("unable to fetch the commit");
        let oid = self.repo.merge_base(head_commits.id(), other_branch_commits.id()).unwrap();
        self.repo.find_commit(oid)
    }
    /// If you want to have the cimmits of both branches run this function
    #[allow(unused_must_use)]
    fn merge_trees(&mut self) {
        let src_branch=self.repo.head().expect("unable to get the head");

        let src_branch_commit=src_branch.peel_to_commit().expect("unable to fetch the commit");
        let src_branch_tree=src_branch_commit.tree().expect("unable to fetch the tree");

        let other_branch=self.repo.find_branch(&self.branches.dest_branch,
            git2::BranchType::Local).expect("unable to fetch other branch").into_reference();

        let other_branch_tree=other_branch.peel_to_commit()
            .expect("unable to fetch the commit in the dest branch").tree().expect("unable to fetch the tree in the dest branch");

        let ancestor=self.find_ancesistor().expect("There is no common parent between those commits");

        let ancestor_tree=ancestor.tree().unwrap();

        let mut merged_options=MergeOptions::default();
        // let mut checkout_builder=CheckoutBuilder::default();

        // The below trees are conflicted 
        let merged_index=self.repo.merge_trees(
            &ancestor_tree,
            &src_branch_tree,
            &other_branch_tree,
            Some(merged_options.patience(true))).unwrap();
        let conflicts=merged_index.conflicts().unwrap();
        // the above index is created but its not connected to a repostiroy
        let mut index=Index::new().unwrap();
        conflicts.map(|conf|{
            let entry=conf.unwrap();
            let ancestor=entry.ancestor.unwrap();
            let base=entry.our.unwrap();
            index.add(&self.make_entry(ancestor, base, true));
        });
        // Apply the index changes to the repository
        // TODO: Separate the function and make it smaller
        self.apply_index_changes(index);

        let new_tree=self.repo.find_tree(self.index.write_tree().unwrap()).unwrap();
        let signature=self.repo.signature().unwrap().to_owned();
        let message=format!("Resolve Conflict through tree resolution:  {} branch into {} branch", self.branches.src_branch, self.branches.dest_branch);
        // get the heads commits
        let parents_commits=&[&src_branch_commit, &ancestor];
        //rust git2 doesn't automatically clean up the conflict the conflict must be deleted
        match self.repo.commit(Some("HEAD"), &signature, &signature, &message, &new_tree, parents_commits){
            Ok(_val) => {
                //after making the commit git must know that the commit is clearing the conflict
                //therefore, MERGE_HEAD file must be deleted to indicate the success of the merge
                let merge_head_path=self.repo.path().join("MERGE_HEAD"); // I believe this is the
                                                                         // line where the error
                                                                         // stems
                //repo.path outputs the content of the .git directory
                //join "MERGE_HEAD" finds the file that starts with MERGE HEAD
                if merge_head_path.exists(){
                    fs::remove_file(merge_head_path).expect("unable to remove the file");
                    //deleting the merge conflict if the commit didn't auto delete
                }
                println!("conflict is resolved");
            },
            _=> println!("Commit is not successful"),
        }
    }
    //Making a commit
    //this function has an embedding implementation
    #[allow(unused_must_use)]
    fn commit(&mut self, mut index: Index, parent_commits: &[&Commit], msg: String)-> bool{
        // let _=self.index.write();
        index.write();
        let tree=self.repo.find_tree(self.index.write_tree().unwrap()).unwrap();
        let signature=self.repo.signature().unwrap().to_owned();
        // let message=format!("Resolve Conflict: Merge {} branch into {} branch", self.branches.src_branch, self.branches.dest_branch);

        // get the heads commits

        // let head=self.repo.head().unwrap();
        // // retreive the commits of "ours" branch
        // let ours_parents_commits=head.peel_to_commit().expect("error peeling to commit in ours version");
        // let theirs=self.repo.find_reference("MERGE_HEAD").expect("unable to find the second theirs reference");
        // // retreive the commits of "theirs" branch
        // let theirs_parents_commits=theirs.peel_to_commit().expect("error peeling to a commit in theirs version");
        // let parents_commits=&[&ours_parents_commits, &theirs_parents_commits];

        //rust git2 doesn't automatically clean up the conflict the conflict must be deleted
        match self.repo.commit(Some("HEAD"), &signature, &signature, &msg, &tree, parent_commits){
            Ok(_val) => {
                //after making the commit git must know that the commit is clearing the conflict
                //therefore, MERGE_HEAD file must be deleted to indicate the success of the merge
                let merge_head_path=self.repo.path().join("MERGE_HEAD"); // I believe this is the
                                                                         // line where the error
                                                                         // stems
                //repo.path outputs the content of the .git directory
                //join "MERGE_HEAD" finds the file that starts with MERGE HEAD
                if merge_head_path.exists(){
                    fs::remove_file(merge_head_path).expect("unable to remove the file");
                    //deleting the merge conflict if the commit didn't auto delete
                }
                true},
            _Error => false,
        }
    }

    //return the file with conditions  
    //this function has an embedding implementation
    fn return_conflicted_files(&self,condition: Status)-> Option<Vec<String>>{
        let mut options=StatusOptions::new();
        options.include_untracked(false).recurse_untracked_dirs(false);
        let status=self.repo.statuses(Some(&mut options)).unwrap();
        let mut list_of_conflicted_files=Vec::new();
        for i in status.iter(){
            if i.status().contains(condition) && let Some(path)=i.path(){
                list_of_conflicted_files.push(path.to_owned());
            }
        }
        Some(list_of_conflicted_files)
    }
    //Merge function

    fn merge(&self,branch_1_commit: Commit, branch_2_commit: Commit) -> Result<Index, Error>{
        let merge_options=MergeOptions::new();
        match self.repo.merge_commits(&branch_1_commit,&branch_2_commit,Some(&merge_options)) {
            Ok(index) =>{
                Ok(index)
            },
            Error => {
                Error
            },
        }
    }

    // In most cases this is the prefered choice for programmers
    // The checkout is the first step towards changing the index
    fn checkout_version(&mut self, ours: bool) -> &mut Self{
        let head_branch= self.repo.head()
            .expect("unable to return the reference")
            .shorthand()
            .expect("unable to retrieve the branch namepointed by the head")
            .to_string();
        //checking the branch pointed by the head to build the checkout
        if head_branch!=self.branches.src_branch && head_branch!=self.branches.dest_branch{
           panic!("head is not pointing to any branch"); 
        }
        else {
            match ours{
                true => self.builder.use_ours(true),
                false => self.builder.use_theirs(true),
            };
        }
        self
    }

    //this function has an embedding implementation
    #[allow(unused_must_use)]
    fn checkout_files(&mut self) -> Vec<String>{
        //add files paths to be checked out with the new merge 
        let files=self.return_conflicted_files(Status::CONFLICTED).expect("files cannot be found");
        // specify the files for which the checkout is to be held for
        files.iter().map(|x| {
           //the below function adds the files to the checkout builder 
           self.builder.path(x).force();
        }).collect::<Vec<_>>();
        files
    }

    //resolves the conflict between two branches by discarding the changes of either two branches
    fn resolve_conflict_by_discarding(&mut self){
        let files=self.checkout_files();
        let _=self.repo.checkout_index(Some(&mut self.index), Some(&mut self.builder));//revert back the index to match the index to the checkout builder
        self.staging(files); //stage the changes
        match self.commit(){//commit the changes
            true => println!("conflict is resolved"),
            false => panic!("error resolving the conflict"),
        }
    }

    fn does_conflict_exists(&self) -> bool{
        self.index.has_conflicts()
    }
    //resolve conflict by merging the changes from both branches : e.g. delete the conflict
    //markers
    fn remove_conflict_markers(&self, file_name: String){
        let file_path=fs::read_to_string(&file_name).unwrap();
        let modify_content=file_path.lines().filter(
            |x| !x.contains("<<<<<<<") & !x.contains("======") & !x.contains(">>>>>>")).collect::<Vec<_>>().join("\n");
        let _=fs::write("tempfile", modify_content);
        let _=fs::rename("tempfile", file_name);
        let _=fs::remove_file("tempfile");
    }

    // This function merge the contents of the conflicted branches
    fn merge_files(&mut self) -> Vec<String>{
        // return the conflicted files
        let files=self.return_files(Status::CONFLICTED).expect("files cannot be found");
        // merge the contents of each file from the conflicted branches
        let _=files.iter().map(|x| {
            Self::remove_conflict_markers(self, x.to_string());
        }).collect::<Vec<_>>();
        files
    }

    //resolves the conflict between two branches by combining the changes of both branches
    fn resolve_conflict_by_combining(&mut self){
        let files=self.merge_files();
        self.staging(files); //stage the changes
        match self.commit(){//commit the changes
            true => println!("conflict is resolved"),
            false => panic!("error resolving the conflict"),
        }
    }
}
