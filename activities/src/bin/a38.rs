// Topic: Multithreading
//
// Requirements:
// * Run the provided functions in threads
// * Retrieve the data from the threads to print the message
//   "Hello, threads!"
//
// Notes:
// * Use the join function to wait for threads to finish

use std::thread;

fn msg_hello() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "Hello, "
}

fn msg_thread() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "threads"
}

fn msg_excited() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "!"
}

fn main() {
  let hello_thread = thread::spawn(move || msg_hello());
  let threads_thread = thread::spawn(move || msg_thread());
  let excited_thread = thread::spawn(move || msg_excited());

  let hello_res = hello_thread.join();
  let threads_res = threads_thread.join();
  let excited_res = excited_thread.join();

  match hello_res {
    Err(_) => println!("Error here."),
    Ok(hey) => match threads_res {
      Err(_) => println!("Error here."),
      Ok(threads) => match excited_res {
        Err(_) => println!("Error here."),
        Ok(excited) => println!("{hey}{threads}{excited}")
      }
    }
  }
}
