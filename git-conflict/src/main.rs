use git2::{Index, Repository};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::env;
fn main(){
    let dir=env::current_dir().unwrap();
    return_path(dir.as_path());
}


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

#[allow(non_snake_case)]
fn return_index(repo: Repository) -> Option<Index,>{
    match repo.index(){
        Ok(index) => {
            Some(index)
        },
        Error => None,
    }

}
