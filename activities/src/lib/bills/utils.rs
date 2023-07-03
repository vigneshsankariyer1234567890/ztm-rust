pub fn split_input_by_char(input: &str, character: char) -> Vec<String> {
  input.split(character)
    .map(|s| s.to_string())
    .collect()
}
