use git2::{Error, Repository};
use std::fs::File;
use std::io::Write;
fn main(){
    let path= "../../../graphs-rust/graphs-lib/benches/my_bench.rs";
    match Repository::discover(path){
        Ok(repo) => println!("Repo found in this directory: {:?}", repo.workdir()),
        Error=>println!("path is not found"),
    }

}
