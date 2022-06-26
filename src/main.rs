#![allow(unused)]

use clap::Parser;
use dialoguer::Confirm;
use dialoguer::Input;
use std::fs;
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

    let component_file_name = component_name.clone() + ".tsx";

    let component_template = fs::read_to_string("./templates/Component.template")
        .unwrap()
        .replace("<<COMPONENT_NAME>>", &component_name);

    let file = create_file(
        [
            component_name.clone(),
            component_name + &component_file_name,
        ]
        .iter()
        .collect::<PathBuf>(),
    )
    .write(component_template.as_bytes());

    let with_tests = Confirm::new()
        .with_prompt("Tests?")
        .default(true)
        .interact()
        .unwrap();

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
