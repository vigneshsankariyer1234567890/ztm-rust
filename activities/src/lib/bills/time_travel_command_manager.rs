use std::convert::TryInto;
use super::command::command_type::{Command, CommandType};
use super::command::redo_command::RedoCommand;
use super::command::undo_command::UndoCommand;

fn generate_undo_command(command_pointer: usize, command_stack: &Vec<Box<dyn Command>>) -> Result<Box<dyn Command>, String> {
  println!("List of commands: {:?}\n", command_stack);
  println!("Given pointer: {:?}\n", command_pointer);

  if command_pointer == 0 {
    return Err("Couldn't complete Undo action, at oldest version.".to_string());
  }

  // look for first command from command_pointer reversed which is a crud command
  let optional = command_stack.iter()
    .enumerate()
    .rev()
    .filter(|(index, _value)| *index <= command_pointer.try_into().unwrap())
    .find_map(|(i, command)| command.as_crud_command().and_then(|c| Some((c, i))));

  println!("found command: {:?}", optional);

  match optional {
    None => Err(
      "No undoable command found, please try again".to_string()
    ),
    Some((command, i)) => Ok(
      Box::new(UndoCommand::of(command, i))
    )
  }
}

fn generate_redo_command(command_pointer: usize, command_stack: &Vec<Box<dyn Command>>) -> Result<Box<dyn Command>, String> {
  if command_pointer >= command_stack.len() {
    // pointing to latest element, no more elements
    return Err("No redoable action found, at latest version.".to_owned());
  }

  // look for first command from command_pointer to command_stack.len() which is a crud command
  let optional = command_stack.iter()
    .enumerate()
    .filter(|(index, _value)| *index >= command_pointer)
    .find_map(|(i, command)| command.as_crud_command().and_then(|c| Some((c, i))));

  println!("Command to redo: {:?}\n\n", optional);

  match optional {
    None => Err(
      "Couldn't complete Redo action, something went wrong".to_string()
    ),
    Some((command, i)) => Ok(
      Box::new(RedoCommand::of(command, i + command_pointer))
    )
  }
}

pub fn generate_time_travel_command(
  command_type: &CommandType,
  command_pointer: usize,
  command_stack: &Vec<Box<dyn Command>>) -> Result<Box<dyn Command>, String> {
  match command_type {
    CommandType::Undo => generate_undo_command(command_pointer, command_stack),
    CommandType::Redo => generate_redo_command(command_pointer, command_stack),
    _ => Err("Incorrect command type given".to_string()),
  }
}