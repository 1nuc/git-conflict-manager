use git2::{Error, Index, Repository, Status, StatusOptions};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::env;
fn main(){
        // match return_index(repo){
        //     Some(index)=> println!("{:?}", index.get(2)),
        //     None => (),
        // }
}
//TODO: Make a tag to know when there is a conflict
//TODO: Detect the conflicted branches
//TODO: make a manual merge

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

fn return_file(condition: Status)-> Option<Vec<String>>{
    let dir=env::current_dir().unwrap();
    if let Some(repo)=return_path(dir.as_path()){
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
    else{
        None
    }
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

