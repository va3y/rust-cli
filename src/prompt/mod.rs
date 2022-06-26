use crate::utils::comma_separate_str::comma_separate_str;
use crate::utils::structs::UserInput;
use dialoguer::Confirm;
use dialoguer::Input;

pub fn run_prompt() -> UserInput {
    let component_name: String = Input::new()
        .with_prompt("What is the component name?")
        .interact_text()
        .unwrap();

    let with_tests = Confirm::new()
        .with_prompt("Tests?")
        .default(true)
        .interact()
        .unwrap();

    let child_components: String = Input::new()
        .with_prompt("Are there any child component? (comma-separated)")
        .interact_text()
        .unwrap();
    let child_components = comma_separate_str(&child_components);

    let child_containers: String = Input::new()
        .with_prompt("Are there any child containers? (comma-separated)")
        .interact_text()
        .unwrap();
    let child_containers = comma_separate_str(&child_containers).as_ref();

    UserInput {
        tests: with_tests,
        child_components: &child_components,
        child_containers,
        root_component_name: component_name.clone(),
    }
}
