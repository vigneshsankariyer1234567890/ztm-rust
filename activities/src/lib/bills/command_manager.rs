use super::command::command_type::*;
use super::bill_manager::BillManager;
use super::command::welcome_message_command::WelcomeMessageCommand;
use super::crud_command_manager::generate_crud_command;
use super::view_command_manager::generate_view_command;
use super::time_travel_command_manager::generate_time_travel_command;
use super::utils::split_input_by_char;
use super::command::exit_command::ExitCommand;
use super::command::help_command::HelpCommand;

#[derive(Debug)]
pub struct CommandManager {
  bill_manager: BillManager,
  command_stack: Vec<Box<dyn Command>>,
  pub command_pointer: usize,
}

impl CommandManager {
  pub fn of(bill_manager: BillManager, command_stack: Vec<Box<dyn Command>>) -> Self {
    Self {
      bill_manager,
      command_stack,
      command_pointer: 0,
    }
  }

  pub fn generate_welcome_message_command() -> Box<dyn ExecutableCommand> {
    Box::new(WelcomeMessageCommand::of())
  }

  pub fn generate_command(&self, input: &str) -> Result<Box<dyn Command>, String> {
    let split_input = split_input_by_char(input, ' ');

    let command_word = split_input[0].as_str();

    let args = split_input[1..].to_vec();

    let command_type = CommandType::get_command_type(command_word);

    match command_type {
      Some(unwrapped_command_type) => match unwrapped_command_type {
        CommandType::Add |
        CommandType::Edit |
        CommandType::Remove => generate_crud_command(
          &unwrapped_command_type, 
          &args
        ),
        CommandType::Redo |
        CommandType::Undo => generate_time_travel_command(
          &unwrapped_command_type,
          self.command_pointer,
          &self.command_stack
        ),
        CommandType::View => generate_view_command(&args),
        CommandType::Help => Ok(Box::new(HelpCommand::of())),
        CommandType::Exit => Ok(Box::new(ExitCommand::of())),
        _ => Err("Inaccessible command given".to_string())
      },
      None => Err("Invalid command given".to_string())
    }
  }

  pub fn execute_command(
    &self,
    boxed_command: &Box<dyn ExecutableCommand>
  ) -> Result<ExecutionResult, String> {
    // try to execute command
    let result = Some(self.bill_manager.clone())
      .map(|bm| boxed_command.execute(bm));

    println!("Your result: {:?}", result);

    // match on result
    match result {
      None => Err("An error occured, please try again.\n".to_string()),
      Some(er) => Ok(er),
    }
  }

  pub fn generate_crud_command_for_time_travel(
    boxed_command: &Box<dyn TimeTravelCommand>
  ) -> (Box<dyn CrudCommand>, usize) {
    (boxed_command.generate_new_crud_command(), boxed_command.get_new_position_of_command())
  }

  pub fn reset_without_committing(
    &mut self,
    optional_bill_manager: Option<BillManager>,
    new_command_pointer: usize
  ) -> Result<String, String> {
    if new_command_pointer > self.command_stack.len() {
      return Err("Unable to save results\n".to_string());
    }

    if optional_bill_manager.is_none() {
      return Err("Unable to save results\n".to_string());
    }

    self.bill_manager = optional_bill_manager.unwrap();

    self.command_pointer = new_command_pointer;

    Ok("Results saved\n".to_string())
  }

  pub fn commit_results(
    &mut self,
    optional_bill_manager: Option<BillManager>,
    boxed_command: &Box<dyn ExecutableCommand>,
    new_command_pointer: usize
  ) -> Result<String, String>{
    println!("pointer value given: {:?}\n\n", new_command_pointer);
    if new_command_pointer > self.command_stack.len() + 1 {
      return Err("Unable to save results\n".to_string());
    }

    if optional_bill_manager.is_none() {
      return Err("Unable to save results\n".to_string());
    }

    self.bill_manager = optional_bill_manager.unwrap();

    // one-indexed
    self.command_stack.truncate(new_command_pointer - 1);

    // push the new command to the stack
    self.command_stack.push(boxed_command.as_command());

    // set command_pointer to be new_command_pointer
    self.command_pointer = new_command_pointer;

    Ok("Results saved\n".to_string())
  }
}