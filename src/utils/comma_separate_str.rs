pub fn comma_separate_str(input: &String) -> Vec<&str> {
    input.split(",").collect::<Vec<&str>>()
}
