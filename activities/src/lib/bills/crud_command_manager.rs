use super::command::command_type::{Command, CommandType};
use super::command::add_command::AddCommand;
use super::command::edit_command::EditCommand;
use super::command::remove_command::RemoveCommand;
use super::bill::Bill;
use super::utils::split_input_by_char;

pub fn generate_crud_command(
  command_type: &CommandType,
  input_array: &Vec<String>
) -> Result<Box<dyn Command>, String> {
  if input_array.len() > 2 {
    return Err("Incorrect arguments provided.\n".to_string());
  }

  let name = input_array.get(0).map(|s| s.as_str());
  if name.is_none() {
    return Err("Incorrect arguments provided.\n".to_string());
  }

  let unwrapped_name = name.unwrap();

  match command_type {
    CommandType::Remove => Ok(Box::new(RemoveCommand::of(unwrapped_name.to_string()))),
    CommandType::Add | CommandType::Edit => generate_add_or_edit_command(input_array, unwrapped_name, command_type),
    _ => {
      return Err("Invalid command passed".to_string())
    }
  }
}

fn generate_add_or_edit_command(
  input_array: &Vec<String>, 
  unwrapped_name: &str,
  command_type: &CommandType
) -> Result<Box<dyn Command>, String> {
  match command_type.clone() {
    CommandType::Add | CommandType::Edit => (),
    _ => return Err("Invalid command passed".to_string())
  };

  let bill_amount_as_string = input_array.get(1).map(|s| s.as_str());

  if bill_amount_as_string.is_none() {
    return Err("Incorrect arguments provided.\n".to_string());
  }

  let unwrapped_bill_amount_as_string = bill_amount_as_string.unwrap();

  let split_input = split_input_by_char(unwrapped_bill_amount_as_string, '.');

  let bill = Bill::generate_bill(unwrapped_name, &split_input);

  if bill.is_none() {
    return Err("Couldn't generate bill".to_string());
  }

  // can only be either Add or Edit by match
  if command_type.clone() == CommandType::Add {
    return Ok(Box::new(AddCommand::of(bill.unwrap())));
  }

  Ok(Box::new(EditCommand::of(bill.unwrap())))
}