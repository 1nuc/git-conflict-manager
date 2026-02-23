use git2::{Commit, Error, Index, MergeOptions, Repository, Status, StatusOptions, build::CheckoutBuilder};
use crate::GitOps;
use std::{env,path::{Path, PathBuf}};

struct Branches{
    src_branch: String,
    dest_branch: String,
}

impl Branches{
   fn init(branch_1: &str, branch_2: &str) -> Self{
       Branches{
           src_branch: branch_1.to_string(),
           dest_branch: branch_2.to_string(),
       }
   } 
}
//creating a struct that contains the essential details for a branch 
pub struct Repo<'a>{
    path: PathBuf,
    repo: Repository,
    index: Index,
    branches: Branches,
    builder: CheckoutBuilder<'a>,
}

#[allow(non_snake_case)]
impl <'a>Repo<'a>{
    //init
    fn init(branch_1: &str, branch_2: &str) -> Self{
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
    //TODO: Returning the directory path
    fn return_repo(file_path: PathBuf) -> Option<Repository, > {
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

    //TODO: staging changes
    //this function has an embedding implementation
    fn staging(&mut self, files: Vec<String>){
        let _=files.iter().map(|x| {
            let path=Path::new(x);
            self.index.add_path(path).expect("Error adding the file to the staging area");
            }).collect::<Vec<_>>();
    }
    //TODO: Making a commit

    //this function has an embedding implementation
    fn commit(&mut self)-> bool{
        let _=self.index.write();
        let tree=self.repo.find_tree(self.index.write_tree().unwrap()).unwrap();
        let signature=self.repo.signature().unwrap().to_owned();
        let message="merge conflict";
        // get the heads commits
        let head=self.repo.head().unwrap();
        let parents_commits=&[&head.peel_to_commit().unwrap()];

        match self.repo.commit(Some("HEAD"), &signature, &signature, message, &tree, parents_commits){
            Ok(_val) => true,
            _Error => false,
        }
    }

    //TODO: return the file with conditions  
    //this function has an embedding implementation
    fn return_files(&self,condition: Status)-> Option<Vec<String>>{
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
    //TODO: Merge function

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
    fn checkout_local(&mut self){
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
            self.builder.use_ours(true);
        }
    }


    // A rarely used but useful option
    fn checkout_foreign(&mut self){
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
            self.builder.use_theirs(true);
        }
    }

    //this function has an embedding implementation
    fn checkout_files(&mut self) -> Vec<String>{
        //add files paths to be checked out with the new merge 
        let files=self.return_files(Status::CONFLICTED).expect("files cannot be found");
        // specify the files for which the checkout is to be held for
        let _=files.iter().map(|x| {
            let _=self.builder.path(x).force();
        }).collect::<Vec<_>>();
        files
    }

    //resolves the conflict between two branches by discarding the changes of either two branches
    fn resolve_conflict_by_discarding(&mut self){
        let _=self.repo.checkout_index(Some(&mut self.index), Some(&mut self.builder));//revert back the index to match the index to the checkout builder
        let files=self.checkout_files();
        self.staging(files); //stage the changes
        match self.commit(){//commit the changes
            true => println!("conflict is resolved"),
            false => panic!("error resolving the conflict"),
        }
    }
    //TODO: resolve conflict by merging the changes from both branches : e.g. delete the conflict
    //markers

    // fn testing_conflict_detection(){
    //     let args: Vec<String>=env::args().collect();
    //     // let branch_1=args[1].clone();
    //     // let branch_2=args[2].clone();
    //     let dir=env::current_dir().unwrap();
    //     if let Some(repo)=return_path(dir.as_path()){
    //         let mut index=repo.index().unwrap();
    //         if index.has_conflicts(){
    //             let repository=Arc::new(repo);
    //             let mut builder=CheckoutBuilder::new();
    //             let checkout_builder=builder.use_ours(true);//specify the checkout build options to use
    //                                                         //the ours (head) reference for the version
    //                                                         //control switching
    //             // resolve_conflicts(index, repo);
    //         }
    //     }
    // }
}
