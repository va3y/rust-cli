#![allow(unused)]
pub mod prompt;
pub mod utils;
use crate::prompt::run_prompt;
use crate::utils::comma_separate_str::comma_separate_str;
use crate::utils::constants::COMPONENT_TOKEN;
use crate::utils::structs::Cli;
use clap::Parser;
use std::fs;
use std::fs::DirBuilder;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use utils::structs::UserInput;

fn create_file(path: PathBuf) -> File {
    File::options()
        .append(true)
        .create(true)
        .open(path)
        .unwrap()
}

fn create_root_component(user_input: &UserInput) {
    let component_name = &user_input.root_component_name;
    DirBuilder::new().create(component_name).unwrap();

    let component_file_name = component_name.clone() + ".tsx";

    let component_template = fs::read_to_string("./templates/Component.template")
        .unwrap()
        .replace(COMPONENT_TOKEN, component_name);

    let file = create_file(
        [component_name.clone(), component_file_name.clone()]
            .iter()
            .collect::<PathBuf>(),
    )
    .write(component_template.as_bytes());
}

fn main() {
    let args = Cli::parse();

    let user_input = run_prompt();
    create_root_component(&user_input);
}
