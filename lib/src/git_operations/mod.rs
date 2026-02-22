use git2::{Commit, Error, Index, MergeOptions, Repository, Status, StatusOptions, build::CheckoutBuilder};
use std::{env,path::{Path, PathBuf},sync::Arc};
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
struct Repo{
    path: PathBuf,
    repo: Repository,
    index: Index,
    branches: Branches,
}

#[allow(non_snake_case)]
impl Repo{
    //init
    fn init(branch_1: &str, branch_2: &str) -> Self{
        let file_path=Self::return_path();
        let Repo=Self::return_repo(file_path).expect("unable to find a git repository");
        let repo_path=Repo.workdir().expect("unable to find the repository path").to_path_buf();
        let Index=Repo.index().expect("unable to find the index");
        Self{
            path: repo_path,
            repo: Repo,
            index: Index,
            branches: Branches::init(branch_1, branch_2),
        }
    }

    //TODO: return the directory as an environment variable
    fn return_path() -> PathBuf{
        let path=env::current_dir().unwrap();
        path
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

impl Repo{

    //TODO: staging changes

    fn staging(&mut self, files: Vec<String>){
        files.iter().map(|x| {
            let path=Path::new(x);
            self.index.add_path(path).expect("Error adding the file to the staging area");
            }).collect::<Vec<_>>();
    }
    //TODO: Making a commit

    fn commit(&self,mut index: Index, repo: Arc<Repository>)-> bool{
        let _=index.write();
        let tree=repo.find_tree(index.write_tree().unwrap()).unwrap();
        let signature=repo.signature().unwrap().to_owned();
        let message="merge conflict";
        // get the heads commits
        let head=repo.head().unwrap();
        let parents_commits=&[&head.peel_to_commit().unwrap()];

        match repo.commit(Some("HEAD"), &signature, &signature, message, &tree, parents_commits){
            Ok(_val) => true,
            Error => false,
        }
    }
    //TODO: return the file with conditions  

    fn return_files(condition: Status, repo: Arc<Repository>)-> Option<Vec<String>>{
        let mut options=StatusOptions::new();
        options.include_untracked(false).recurse_untracked_dirs(false);
        let status=repo.statuses(Some(&mut options)).unwrap();
        let mut list_of_conflicted_files=Vec::new();
        for i in status.iter(){
            if i.status().contains(condition) && let Some(path)=i.path(){
                list_of_conflicted_files.push(path.to_owned());
            }
        }
        Some(list_of_conflicted_files)
    }
    //TODO: Merge function

    fn merge(repo: Repository,branch_1_commit: Commit, branch_2_commit: Commit) -> Result<Index, Error>{
        let merge_options=MergeOptions::new();
        match repo.merge_commits(&branch_1_commit,&branch_2_commit,Some(&merge_options)) {
            Ok(index) =>{
                Ok(index)
            },
            Error => {
                Error
            },
        }
    }
    //TODO: a function to show merge options

    fn testing_conflict_detection(){
        let args: Vec<String>=env::args().collect();
        // let branch_1=args[1].clone();
        // let branch_2=args[2].clone();
        let dir=env::current_dir().unwrap();
        if let Some(repo)=return_path(dir.as_path()){
            let mut index=repo.index().unwrap();
            if index.has_conflicts(){
                let repository=Arc::new(repo);
                let mut builder=CheckoutBuilder::new();
                let checkout_builder=builder.use_ours(true);//specify the checkout build options to use
                                                            //the ours (head) reference for the version
                                                            //control switching
                let repo=Arc::clone(&repository);
                let files=return_files(Status::CONFLICTED, repository).unwrap();
                // specify the files for which the checkout is to be held for
                files.iter().map(|x| {
                    let _=checkout_builder.path(x).force();
                }).collect::<Vec<_>>();
                let _=repo.checkout_index(Some(&mut index), Some(checkout_builder));//revert back the
                                                                                    //current index to
                                                                                    //the index built
                                                                                    //from (head)
                staging(&mut index, files); //stage the changes
                match commit(index, repo){//commit the changes
                    true => println!("conflict is resolved"),
                    false => println!("error resolving the conflict"),
                }
                // resolve_conflicts(index, repo);
            }
        }
    }
}
