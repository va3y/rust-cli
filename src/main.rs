#![allow(unused)]

use clap::Parser;
use dialoguer::Confirm;
use dialoguer::Input;
use std::fs::DirBuilder;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::result::Result;

#[derive(Parser)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn create_file(path: PathBuf) -> File {
    File::options()
        .append(true)
        .create(true)
        .open(path)
        .unwrap()
}

fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    let component_name: String = Input::new()
        .with_prompt("What is the component name?")
        .interact_text()
        .unwrap();

    DirBuilder::new().create(&component_name).unwrap();

    let path: PathBuf = [component_name.clone(), component_name + ".ts"]
        .iter()
        .collect();
    let file = create_file(path);

    if Confirm::new()
        .with_prompt("Do you want to continue?")
        .interact()
        .unwrap()
    {
        println!("Looks like you want to continue");
    } else {
        println!("nevermind then :(");
    }

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
