use colored::*;
use core::panic;
use git_conflict::{GitOps, Initialize, git_src::Repo};
use log::*;
use std::{env, io, process::exit};

fn option_panel(welcome_msg: &str, msg: &str) -> String {
    let options = [
        "Keep Local Head Changes",
        "Keep Foreign Branch Changes",
        "Remove Markers and Keep Both Changes (Soon)",
        "Merge Trees",
        "Exit",
    ];
    println!("{},\n{}: ", welcome_msg, msg);
    let _ = options
        .iter()
        .enumerate()
        .map(|(i, x)| {
            println!("Option {}: {}", 1 + i, x.italic().blue().bold());
        })
        .collect::<Vec<_>>();
    println!("Select the option number: ");
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("error reading the line");
    line
}
fn show_example() {
    warn!(
        "{}",
        "Example: cargo r src_branch dest_branch"
            .italic()
            .bold()
            .yellow()
    );
    warn!(
        "{}",
        "src_branch is the branch is the branch you are currently at whcih is pointed by head"
            .italic()
            .bold()
            .yellow()
    );
    warn!(
        "{}",
        "to check for your source branch type git status"
            .italic()
            .bold()
            .yellow()
    );
    warn!(
        "{}",
        "dest_branch is the branch you are trying to merge"
            .italic()
            .bold()
            .yellow()
    );
    warn!(
        "{}",
        "rewrite the command with specifying the name of the branches"
            .italic()
            .bold()
            .yellow()
    );
}
fn checking_value(value: i32) -> bool {
    value < 6 && value > 0
}

fn parent_interference_check() -> Option<bool> {
    let mut line = String::new();
    println!("{}", "Parenet Interference? ".italic().bold().green());
    println!(
        "{}",
        "For example: if the head branch latest commit is -add features x-"
            .italic()
            .bold()
            .bright_yellow()
    );
    println!(
        "{}",
        "And the incoming branch commit is -fix feature x-"
            .italic()
            .bold()
            .bright_yellow()
    );
    println!(
        "{}",
        "And the ancestor commit of branches is -ship feature x-"
            .italic()
            .bold()
            .bright_yellow()
    );
    println!("{}","The new merge commit will combine the latest cleanest path (ancestor commit) to the new accepted changes".italic().bold().bright_yellow());
    println!(
        "{}",
        "Enter only Yes or No: ".italic().bold().bright_yellow()
    );
    io::stdin()
        .read_line(&mut line)
        .expect("Error reading the line");
    while line.trim_end().to_lowercase().as_str() != "yes"
        && line.trim_end().to_lowercase().as_str() != "no"
    {
        line.clear();
        println!(
            "{}",
            "You have to only enter Yes or No: "
                .italic()
                .bold()
                .purple()
        );
        io::stdin()
            .read_line(&mut line)
            .expect("error reading the line");
    }
    match line.trim_end().to_lowercase().as_str() {
        "yes" => Some(true),
        "no" => Some(false),
        _ => None,
    }
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
fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!(
            "{}",
            "You have to specify the names of the conflicted branches"
                .italic()
                .bold()
                .red()
        );
        show_example();
    }

    let git_control = Repo::init(args[1].clone(), args[2].clone());

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

    let mut line = option_panel(
        &welcome_msg,
        "which conflict resolution you would like to choose",
    );
    while line.trim_end().parse::<i32>().is_err() {
        line = option_panel(&welcome_msg, "Error You should only a valid option");
    }
    let mut opt = line.trim_end().parse::<i32>().unwrap();
    while !checking_value(opt) {
        line = option_panel(&welcome_msg, "You should only select a valid number");
        opt = line.trim_end().parse::<i32>().unwrap();
    }
    match opt {
        1 => {
                git_control.call_discarding(true);
        }
        2 => {
                git_control.call_discarding(false);
        }
        3 => {
            git_control.call_combinition();
        }
        4 => {
            if let Some(parent_interference) = parent_interference_check() {
                if let Some(version) = version_check() {
                    git_control.call_tree_merge(version, parent_interference);
                } else {
                    panic!("Error occured in getting the version value");
                }
            } else {
                panic!("Error occured in getting the parent value");
            }
        }
        5 => {
            exit(0);
        }
        _ => warn!("undefined error"),
    }
}
