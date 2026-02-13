use git2::{Error, Repository};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::env;
fn main(){
    let dir=env::current_dir().unwrap();
    return_path(dir.as_path());
}


#[allow(non_snake_case, unused_variables)]
fn return_path(file_path: &Path){
   match Repository::discover(file_path){
       Ok(repo) => {
           if let Some(path)=repo.workdir(){
               let path_: &Path= path;
               println!("Working repository: {:?}", path_);
           }
       }
       Error => println!("Unable to find the repository path"),
   }
}
