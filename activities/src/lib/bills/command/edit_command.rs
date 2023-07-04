use super::command_type::{Command, CommandType, CrudCommand, ExecutableCommand, ExecutionResult};
use crate::bills::bill::Bill;
use crate::bills::bill_manager::BillManager;

#[derive(Debug, Clone)]
pub struct EditCommand {
  bill: Bill,
  command_type: CommandType,
  previous_bill: Option<Bill>,
}

impl EditCommand {
  pub fn of(bill: Bill) -> Self {
    EditCommand {
      bill,
      command_type: CommandType::Edit,
      previous_bill: None,
    }
  }

  pub fn get_dummy_command() -> Self {
    Self::of(Bill {
      name: String::from(""),
      amount_in_dollars: 0,
      amount_in_cents: 0,
    })
  }

  fn with_previous_bill(mut self, previous_bill: Bill) -> Self {
    self.previous_bill = Some(previous_bill);
    self
  }
}

impl Command for EditCommand {
  fn get_info(&self) -> String {
    "Edit a bill in the manager".to_owned()
  }

  fn get_command_args(&self) -> String {
    "<Name> <New value in dollars>.<New value in cents>".to_owned()
  }

  fn get_command_word(&self) -> String {
    CommandType::Edit.as_str().to_owned()
  }

  fn get_command_type(&self) -> CommandType {
    self.command_type
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

impl ExecutableCommand for EditCommand {
  fn execute(&self, bill_manager: BillManager) -> ExecutionResult {
    let result = bill_manager.edit_bill(&self.bill);

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

impl CrudCommand for EditCommand {
  fn get_inverse(&self) -> Box<dyn CrudCommand> {
    let previous_bill = self.previous_bill.clone().unwrap();
    Box::new(
      EditCommand::of(previous_bill.clone())
    )
  }

  fn clone_boxed_crud(&self) -> Box<dyn CrudCommand> {
    Box::new(self.clone())
  }

  fn with_possibly_previous_bill(&self, optional_bill: Option<Bill>) -> Box<dyn CrudCommand> {
    match optional_bill {
      None => Box::new(self.clone()),
      Some(bill) => Box::new(self.clone().with_previous_bill(bill))
    }
  }
}
