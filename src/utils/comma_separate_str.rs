pub fn comma_separate_str(input: String) -> Vec<&'static str> {
    let res = input.split(",").collect::<Vec<&str>>().to_owned();
    res
}
