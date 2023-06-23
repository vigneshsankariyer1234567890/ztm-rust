use super::command_type::{Command, CommandType};
use super::add_command::AddCommand;
use super::edit_command::EditCommand;
use super::remove_command::RemoveCommand;
use super::view_command::ViewCommand;
use crate::bills::bill_manager::BillManager;


pub struct HelpCommand {
  command_type: CommandType,
}

impl HelpCommand {
  pub fn of() -> Self {
    HelpCommand{
      command_type: CommandType::Help,
    }
  }

  pub fn get_dummy_command() -> Self {
    Self::of()
  }

  fn print_command_list() {
    let commands: Vec<Box<dyn Command>> = vec![
      Box::new(AddCommand::get_dummy_command()),
      Box::new(EditCommand::get_dummy_command()),
      Box::new(HelpCommand::get_dummy_command()),
      Box::new(RemoveCommand::get_dummy_command()),
      Box::new(ViewCommand::get_dummy_command()),
    ];

    println!("{:<10} {:<30} {:<15}", "Command", "Description", "Usage");

    let mapped: Vec<(String, String, String)> = commands.into_iter()
      .map(|cmd| (
        cmd.get_command_word(),
        cmd.get_info(),
        cmd.get_command_args()
      ))
      .collect();

    for tuple in mapped {
      println!("{:<10} {:<30} {:<15}", tuple.0, tuple.1, tuple.2);
    }
  }
}

impl Command for HelpCommand {
  fn execute(&self, bill_manager: BillManager) -> Option<BillManager> {
    HelpCommand::print_command_list();
    
    BillManager::of(bill_manager.get_bill_collection()?)
  }

  fn get_info(&self) -> String {
    "Get help on the commands".to_owned()
  }

  fn get_command_args(&self) -> String {
    "".to_owned()
  }

  fn get_command_word(&self) -> String {
    CommandType::Help.as_str().to_owned()
  }
}