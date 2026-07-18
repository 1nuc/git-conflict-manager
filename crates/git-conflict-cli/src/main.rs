use colored::*;
use core::panic;
use git_conflict::{GitOps, Initialize, git_src::Repo};
use log::*;
use std::{env, io, process::exit};

use crate::tui::App;
mod tui;

fn checking_value(value: i32) -> bool {
    value < 6 && value > 0
}

fn version_check() -> Option<bool> {
    let mut line = String::new();
    println!(
        "{}",
        "which branch to base the index or tree on: (Ours or Theirs): "
            .italic()
            .bold()
            .green()
    );
    println!(
        "{}",
        "Ours is the branch that is pointed by the head"
            .italic()
            .bold()
            .blue()
    );
    println!(
        "{}",
        "Theirs is the other branch that is targeted for merge"
            .italic()
            .bold()
            .blue()
    );
    println!("{}", "Enter only ours or theirs: ".italic().bold().blue());
    io::stdin()
        .read_line(&mut line)
        .expect("Error reading the line");
    while line.trim_end().to_lowercase().as_str() != "ours" && line.trim_end().to_lowercase().as_str() != "theirs" {
        line.clear();
        println!(
            "{}",
            "You have to only enter ours or theirs: "
                .italic()
                .bold()
                .purple()
        );
        io::stdin()
            .read_line(&mut line)
            .expect("error reading the line");
    }
    match line.trim_end().to_lowercase().as_str() {
        "ours" => Some(true),
        "theirs" => Some(false),
        _ => None,
    }
}

#[allow(unused_must_use)]
fn main() -> io::Result<()>{
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    // let git_control = Repo::init(args[1].clone(), args[2].clone());
    let mut app=App::default();
    ratatui::run(|terminal| app.run(terminal))
}
#[allow(unused_must_use)]
fn output(git_control: Repo){

    if !git_control.does_conflict_exists() {
        println!(
            "{}",
            "There is no conflict in your index".italic().bold().red()
        );
        println!("Move Anyway");
        let mut next_move = String::new();
        io::stdin()
            .read_line(&mut next_move)
            .expect("an error occur while fetching the user input");
        next_move.trim_end();
        if next_move == "No" {
            exit(0);
        }
    }
    let welcome_msg = "Git Conflict Manager.... The tool for ultimate file control"
        .italic()
        .bold()
        .bold()
        .green();

    // match opt {
    //     1 => {
    //             git_control.call_discarding(true);
    //     }
    //     2 => {
    //             git_control.call_discarding(false);
    //     }
    //     3 => {
    //         git_control.call_combinition();
    //     }
    //     4 => {
    //         if let Some(parent_interference) = parent_interference_check() {
    //             if let Some(version) = version_check() {
    //                 git_control.call_tree_merge(version, parent_interference);
    //             } else {
    //                 panic!("Error occured in getting the version value");
    //             }
    //         } else {
    //             panic!("Error occured in getting the parent value");
    //         }
    //     }
    //     5 => {
    //         exit(0);
    //     }
    //     _ => warn!("undefined error"),
    // }
}
