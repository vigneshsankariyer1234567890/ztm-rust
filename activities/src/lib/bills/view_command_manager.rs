use super::command::command_type::{Command};
use super::command::view_command::ViewCommand;

pub fn generate_view_command(input_array: &Vec<String>) -> Option<Box<dyn Command>> {
  let optional_id = input_array
    .get(0)
    .map(|id| id.to_owned());

  Some(
    Box::new(ViewCommand::of(optional_id))
  )
}