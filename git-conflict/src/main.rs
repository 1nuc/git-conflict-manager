// use git2::RevertOptions
use std::fs::File;
use std::io::Write;
fn main(){
    let mut file=File::create("test.txt").unwrap();

    file.write_all(b"Hello, world!");
}
