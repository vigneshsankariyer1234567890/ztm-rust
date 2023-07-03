use super::command::command_type::*;
use super::bill_manager::BillManager;
use super::command::welcome_message_command::WelcomeMessageCommand;
use super::crud_command_manager::generate_crud_command;
use super::view_command_manager::generate_view_command;
use super::time_travel_command_manager::generate_time_travel_command;
use super::utils::split_input_by_char;
use super::command::exit_command::ExitCommand;
use super::command::help_command::HelpCommand;

pub struct CommandManager {
  bill_manager: Option<BillManager>,
  command_stack: Vec<Box<dyn Command>>,
  pub command_pointer: i32,
}

impl CommandManager {
  pub fn of(bill_manager: Option<BillManager>, command_stack: Vec<Box<dyn Command>>) -> Self {
    Self {
      bill_manager,
      command_stack,
      command_pointer: -1,
    }
  }

  pub fn generate_welcome_message_command() -> Box<dyn ExecutableCommand> {
    Box::new(WelcomeMessageCommand::of())
  }

  pub fn generate_command(&self, input: &str) -> Option<Box<dyn Command>> {
    let split_input = split_input_by_char(input, ' ');

    let command_word = split_input[0].as_str();

    let args = split_input[1..].to_vec();

    let command_type = CommandType::get_command_type(command_word)?;

    match command_type {
      CommandType::Add => generate_crud_command(&command_type, &args),
      CommandType::View => generate_view_command(&args),
      CommandType::Remove => generate_crud_command(&command_type, &args),
      CommandType::Edit => generate_crud_command(&command_type, &args),
      CommandType::Undo => generate_time_travel_command(&command_type, self.command_pointer, &self.command_stack),
      CommandType::Redo => generate_time_travel_command(&command_type, self.command_pointer, &self.command_stack),
      CommandType::Help => Some(Box::new(HelpCommand::of())),
      CommandType::Exit => Some(Box::new(ExitCommand::of())),
      _ => None
    }
  }

  pub fn execute_command(
    &mut self,
    boxed_command: &mut Box<dyn ExecutableCommand>
  ) -> Result<ExecutionResult, String> {
    // try to execute command
    let result = self.bill_manager
      .as_ref()
      .and_then(|bm| boxed_command.execute((*bm).clone()));

    // match on result
    match result {
      None => Err("An error occured, please try again.".to_string()),
      Some(new_bm) => {
        // Ok over here, can add to 
        Ok(
          ExecutionResult {
            bill_manager: Some(new_bm),
            successful_executable_command: boxed_command.clone_boxed_executable()
          }
        )
      }
    }
  }

  pub fn generate_crud_command_for_time_travel(
    boxed_command: &Box<dyn TimeTravelCommand>
  ) -> (Box<dyn CrudCommand>, i32) {
    (boxed_command.generate_new_crud_command(), boxed_command.get_new_position_of_command())
  }

  pub fn reset_without_committing(
    &mut self,
    optional_bill_manager: Option<BillManager>,
    new_command_pointer: i32
  ) -> Result<String, String> {
    if (new_command_pointer as usize) >= self.command_stack.len() {
      return Err("Unable to save results".to_string());
    }

    if new_command_pointer < 0 {
      return Err("Unable to save results".to_string());
    }

    self.bill_manager = optional_bill_manager;

    self.command_pointer = new_command_pointer;

    Ok("Results saved".to_string())
  }

  pub fn commit_results(
    &mut self,
    optional_bill_manager: Option<BillManager>,
    boxed_command: &Box<dyn ExecutableCommand>,
    new_command_pointer: i32
  ) -> Result<String, String>{
    if (new_command_pointer as usize) >= self.command_stack.len() {
      return Err("Unable to save results".to_string());
    }

    if new_command_pointer < 0 {
      return Err("Unable to save results".to_string());
    }

    self.bill_manager = optional_bill_manager;

    self.command_stack.truncate(new_command_pointer as usize);

    // push the new command to the stack
    self.command_stack.push(boxed_command.clone().as_command());

    // set command_pointer to be new_command_pointer
    self.command_pointer = new_command_pointer;

    Ok("Results saved".to_string())
  }
}