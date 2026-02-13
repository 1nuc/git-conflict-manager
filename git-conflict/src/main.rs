use git2::{Error, Repository};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::env;
fn main(){
    let path=Path::new("../../../../ClearSet/FolioBridge_Source_Code/folio-bridge-ui/src/app/components/actionitemchart/actionitemchart.component.css");
    return_path(path);
    let dir=env::current_dir();
    println!("dir: {:?}", dir);
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
