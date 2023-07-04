use super::command_type::{Command, CommandType, CrudCommand, ExecutableCommand, ExecutionResult};
use super::add_command::AddCommand;
use crate::bills::{bill_manager::BillManager, bill::Bill};
#[derive(Debug, Clone)]
pub struct RemoveCommand {
  id: String,
  command_type: CommandType,
  bill: Option<Bill>
}

impl RemoveCommand {
  pub fn of(id: String) -> Self {
    RemoveCommand {
      id,
      command_type: CommandType::Remove,
      bill: None,
    }
  }

  pub fn get_dummy_command() -> Self {
    Self::of(String::from(""))
  }

  pub fn with_bill(mut self, bill: Bill) -> Self {
    self.bill = Some(bill);
    self
  }
}

impl Command for RemoveCommand {
  fn get_info(&self) -> String {
    "Remove a bill".to_owned()
  }

  fn get_command_args(&self) -> String {
    "<Name of bill>".to_owned()
  }

  fn get_command_word(&self) -> String {
    CommandType::Remove.as_str().to_owned()
  }

  fn get_command_type(&self) -> CommandType {
    self.command_type.clone()
  }

  fn as_command(&self) -> Box<dyn Command> {
    Box::new(self.clone())
  }

  fn as_crud_command(&self) -> Option<Box<dyn CrudCommand>> {
    Some(Box::new(self.clone()))
  }

  fn as_executable_command(&self) -> Option<Box<dyn ExecutableCommand>> {
    Some(Box::new(self.clone()))
  }
}

impl ExecutableCommand for RemoveCommand {
  fn execute(&self, bill_manager: BillManager) -> ExecutionResult {
    let result = bill_manager.remove_bill(&self.id);

    if result.is_none() {
      return ExecutionResult {
        bill_manager: None,
        successful_executable_command: self.clone_boxed_executable()
      };
    }

    let (
      new_bill_manager,
      previous_bill
    ) = result.unwrap();

    let new_command = self
      .with_possibly_previous_bill(Some(previous_bill))
      .clone_boxed_executable();

    ExecutionResult {
      bill_manager: Some(new_bill_manager),
      successful_executable_command: new_command
    }
  }

  fn clone_boxed_executable(&self) -> Box<dyn ExecutableCommand> {
    Box::new(self.clone())
  }
}

impl CrudCommand for RemoveCommand {
  fn get_inverse(&self) -> Box<dyn CrudCommand> {
    Box::new(
      AddCommand::of(self.bill.clone().unwrap())
    )
  }

  fn clone_boxed_crud(&self) -> Box<dyn CrudCommand> {
    Box::new(self.clone())
  }

  fn with_possibly_previous_bill(&self, optional_bill: Option<Bill>) -> Box<dyn CrudCommand> {
    match optional_bill {
      None => Box::new(self.clone()),
      Some(bill) => Box::new(self.clone().with_bill(bill))
    }
  }
}