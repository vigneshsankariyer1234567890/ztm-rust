use super::command::command_type::{Command};
use super::command::view_command::ViewCommand;

pub fn generate_view_command(input_array: &Vec<String>) -> Result<Box<dyn Command>, String> {
  let optional_id = input_array
    .get(0)
    .map(|id| id.to_owned());

  Ok(
    Box::new(ViewCommand::of(optional_id))
  )
}