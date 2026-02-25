use git_conflict::{git_operations::Repo, GitOps, Initialize};
use colored::*;
use log::*;
use std::{env, io};

fn option_panel(welcome_msg: &str, msg: &str) -> String{
    let options=["Keep Local Head Changes", "Keep Foreign Branch Changes", "Remove Markers and Keep Both Changes (Soon)"];
    println!("{},\n{}: ", welcome_msg, msg); 
    let _=options.iter().enumerate().map(|(i,x)| {
        println!("Option {}: {}",1+i,x.italic().blue().bold());
    }).collect::<Vec<_>>();
    println!("Select the option number: ");
    let mut line=String::new();
    io::stdin().read_line(&mut line).expect("error reading the line");
    line
}
fn show_example(){
    warn!("{}","Example: cargo r src_branch dest_branch".italic().bold().yellow());  
    warn!("{}","src_branch is the branch is the branch you are currently at whcih is pointed by head".italic().bold().yellow());
    warn!("{}","to check for your source branch type git status".italic().bold().yellow());
    warn!("{}","dest_branch is the branch you are trying to merge".italic().bold().yellow()); 
    warn!("{}","rewrite the command with specifying the name of the branches".italic().bold().yellow());
}
fn checking_value(value: i32) -> bool{
    value <3 && value >0
}

fn main(){
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();
    let args: Vec<String>=env::args().collect();
    if args.len() <3 {
        println!("{}","You have to specify the names of the conflicted branches".italic().bold().red());
        show_example();
    }

    let mut git_control=Repo::init(args[1].clone(), args[2].clone()); 

    if !git_control.does_conflict_exists(){
        println!("{}","There is no conflict in your index".italic().bold().red());
        return;
    }
    let welcome_msg="Git Conflict Manager.... The tool for ultimate file control".italic().bold().bold().green();

    let mut line=option_panel(&welcome_msg, "which conflict resolution you would like to choose");
    while line.trim_end().parse::<i32>().is_err(){
        line=option_panel(&welcome_msg, "Error You should only a valid option");
    }
    let mut opt=line.trim_end().parse::<i32>().unwrap();
    while !checking_value(opt){
        line=option_panel(&welcome_msg,"You should only select a valid number");
        opt=line.trim_end().parse::<i32>().unwrap();
    }
    match opt{
        1 =>{
            git_control.checkout_local().resolve_conflict_by_discarding();
        },
        2 =>{
            git_control.checkout_foreign().resolve_conflict_by_discarding();
        },
        3 =>{
            println!("feature has not yet been developed");
        },
        _ =>warn!("undefined error"),
    }

}
