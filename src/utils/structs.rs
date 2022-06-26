use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

pub struct UserInput {
    pub tests: bool,
    pub root_component_name: String,
    pub child_containers: &'static Vec<&'static str>,
    pub child_components: &'static Vec<&'static str>,
}
