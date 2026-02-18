use git2::build::CheckoutBuilder;
use git2::{Branch, BranchType, Commit, Error, Index, IndexEntry, MergeOptions, Repository, Status, StatusOptions};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::env;
use std::sync::Arc;
fn main(){
        // match return_index(repo){
        //     Some(index)=> println!("{:?}", index.get(2)),
        //     None => (),
        // }
    // logic();
    testing_conflict_detection();
}
//TODO: Make a tag to know when there is a conflict
//TODO: Detect the conflicted branches
//TODO: make a manual merge
//TODO: Return the commits of the branches to perform the merge
//TODO: detect the conflicted and accept the changes of the defined and passed branch
//TODO: Make a function to make a commit
//TODO: Make a function to show merge options 
//TODO: make a function for staging

#[allow(non_snake_case, unused_variables)]
fn return_path(file_path: &Path) -> Option<Repository, > {
   match Repository::discover(file_path){
       Ok(repo) => {
           if let Some(path)=repo.workdir(){
               let path_: &Path= path;
               Some(repo)
           }
           else{
               println!("no path found for this repo");
               None
           }
       }
       Error => {
           println!("Unable to find the repository path");
           None
       }
   }
}

#[allow(non_snake_case, unused_variables)]
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
#[allow(non_snake_case, unused_variables)]
fn logic(){
    let dir=env::current_dir().unwrap();
    if let Some(repo)=return_path(dir.as_path()){
        // let list_of_conflicted_files=return_files(Status::CONFLICTED, repo);
        let repository = Arc::new(repo);
        let local_branches=repository.branches(Some(BranchType::Local));
        for branch in local_branches.unwrap(){
            let (branch, branch_type)=branch.unwrap();
            // println!("branch: {:?} ", branch.name().unwrap().unwrap());
            // println!("branch Type: {:?}", branch_type);
            let commit=get_the_latest_commit(branch).expect("Error occured while peeling the reference");
            println!("{:?}", commit);
        }
    }
}

#[allow(non_snake_case, unused_variables, unreachable_code)]
fn get_the_latest_commit(branch: Branch) -> Result<Commit, Error>{
    match branch.get().peel_to_commit(){
        Ok(commit) => {
            println!("Reference commit is peeled");
            Ok(commit)
        },
        Error => {
            panic!("Error peeling to commit");
            Error
        } 
    }
}

#[allow(non_snake_case, unused_variables, unreachable_code)]
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


fn resolve_conflicts(mut index: Index,repo: Repository){
    let conflicts: Vec<_>=index.conflicts().unwrap().collect();
    for conflict in conflicts{
        let entry=conflict.unwrap();
        println!("Our :{:?}", entry.our);
        println!("Theirs :{:?}", entry.their);
        let _=index.add(&entry.our.unwrap());
    }
}

fn staging(index: &mut Index, files: Vec<String>){
    files.iter().map(|x| {
        let path=Path::new(x);
        index.add_path(path).expect("Error adding the file to the staging area");
        }).collect::<Vec<_>>();
}

fn commit(mut index: Index, repo: Arc<Repository>)-> bool{
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


#[allow(non_snake_case)]
fn return_index(repo: Repository) -> Option<Index,>{
    match repo.index(){
        Ok(index) => {
            Some(index)
        },
        Error => None,
    }
}
