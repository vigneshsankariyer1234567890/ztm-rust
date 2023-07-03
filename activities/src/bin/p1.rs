use std::io::stdin;

// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.
use vig::bills::bill_manager::BillManager;
use vig::bills::command::command_type::{Command, ExecutionResult, CommandType};
use vig::bills::command_manager::CommandManager;

fn main() {
  let mut command_manager = CommandManager::of(
    Some(
      BillManager::new()
    ),
    Vec::new(),
  );

  let mut welcome_command = CommandManager::generate_welcome_message_command();

  let _ = command_manager.execute_command( &mut welcome_command);
  
  loop {
    // accept input
    println!("Please enter your command");

    let mut input = String::new();
    
    stdin().read_line(&mut input)
      .expect("Failed to read line");

    // parse to command
    let optional_command = command_manager.generate_command(&input);

    // check if exit
    match optional_command.as_ref().map(|c| c.get_command_type()) {
      None => {
        println!("No such command exists, please try again");
        continue;
      },
      Some(CommandType::Exit) => {
        break;
      },
      Some(CommandType::Undo) | Some(CommandType::Redo) => complete_undo_or_redo(&optional_command, &mut command_manager),
      _ => complete_executable_actions(&optional_command, &mut command_manager),
    };
  }

  // exit and say bye over here
  println!("Thanks for using BillManager! Bye!");
  std::process::exit(0);
}

fn complete_executable_actions(optional_command: &Option<Box<dyn Command>>, command_manager: &mut CommandManager) {
    // if not undo or redo, execute and commit results
    let command = optional_command
      .as_ref()
      .and_then(|c| c.as_executable_command());

    let mut executable = command.unwrap().clone_boxed_executable();

    let result = command_manager
      .execute_command(&mut executable)
      .and_then(|r| {
        let ExecutionResult {
          bill_manager,
          mut successful_executable_command,
        } = r;

        command_manager.commit_results(
          bill_manager,
          &mut successful_executable_command,
          command_manager.command_pointer + 1
        )
      });

    match result {
      Ok(_r) => (),
      Err(msg) => println!("Error occured: {:?}", msg)
    }
}

fn complete_undo_or_redo(optional_command: &Option<Box<dyn Command>>, command_manager: &mut CommandManager) {
    let pointer = optional_command
      .as_ref()
      .and_then(|c| c.as_time_travel_command())
      .map(|c| c.get_new_position_of_command())
      .unwrap();

    let mut executable = optional_command
      .as_ref()
      .and_then(|c| c.as_time_travel_command())
      .map(|c| c.generate_new_crud_command())
      .and_then(|c| c.as_executable_command())
      .unwrap();

    let result = command_manager
      .execute_command(&mut executable)
      .and_then(|r| {
        let ExecutionResult {
          bill_manager,
          successful_executable_command: _,
        } = r;

        command_manager.reset_without_committing(
          bill_manager,
          pointer,
        )
      });
        
    match result {
      Ok(_r) => (),
      Err(msg) => println!("Error occured: {:?}", msg)
    }
}
