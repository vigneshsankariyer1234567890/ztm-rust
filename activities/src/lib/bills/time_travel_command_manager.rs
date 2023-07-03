use std::convert::TryInto;
use super::command::command_type::{Command, CommandType};
use super::command::redo_command::RedoCommand;
use super::command::undo_command::UndoCommand;

fn generate_undo_command(command_pointer: i32, command_stack: &Vec<Box<dyn Command>>) -> Option<Box<dyn Command>> {
  if command_pointer < 0 {
    return None;
  }

  // look for first command from command_pointer reversed which is a crud command
  let (command, i) = command_stack.iter()
    .enumerate()
    .rev()
    .filter(|(index, _value)| *index <= command_pointer.try_into().unwrap())
    .find_map(|(i, command)| command.as_crud_command().and_then(|c| Some((c, i))))?;

  Some(Box::new(UndoCommand::of(command, i as i32)))
}

fn generate_redo_command(command_pointer: i32, command_stack: &Vec<Box<dyn Command>>) -> Option<Box<dyn Command>> {
  let last_elem_index = command_stack.len() - 1;
  if command_pointer >= last_elem_index.try_into().unwrap() {
    // pointing to latest element, no more elements
    return None;
  }

  // look for first command from command_pointer to command_stack.len() which is a crud command
  let (command, i) = command_stack.iter()
    .enumerate()
    .filter(|(index, _value)| *index > command_pointer.try_into().unwrap())
    .find_map(|(i, command)| command.as_crud_command().and_then(|c| Some((c, i))))?;

  Some(Box::new(RedoCommand::of(command, i as i32 + command_pointer)))
}

pub fn generate_time_travel_command(
  command_type: &CommandType,
  command_pointer: i32,
  command_stack: &Vec<Box<dyn Command>>) -> Option<Box<dyn Command>> {
  match command_type {
    CommandType::Undo => generate_undo_command(command_pointer, command_stack),
    CommandType::Redo => generate_redo_command(command_pointer, command_stack),
    _ => None,
  }
}