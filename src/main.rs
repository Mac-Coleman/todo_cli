use todo_cli::{Info, TodoItem, fix_input};

fn main() {

    let mut info = Info::new();

    println!("Successfully deserialized todo list.");

    let mut command = String::new();
    fix_input(&mut command);

    while command != "exit" {

        let mut processed_command = command.split(' ');

        match processed_command.next() {
            Some("uncompleted") => {
                println!("Displaying {} uncompleted items.", info.uncompleted.len());
                for index in 0..info.uncompleted.len() {
                    println!("\t{}: {}", index, info.uncompleted[index]);
                }
            },
            Some("completed") => {
                println!("Displaying {} completed items.", info.completed.len());
                for index in 0..info.completed.len() {
                    println!("\t{}: {}", index, info.completed[index]);
                }
            },
            Some("all") => {
                println!("Displaying {} uncompleted items.", info.uncompleted.len());
                for index in 0..info.uncompleted.len() {
                    println!("\t{}: {}", index, info.uncompleted[index]);
                }

                println!("Displaying {} completed items.", info.completed.len());
                for index in 0..info.completed.len() {
                    println!("\t{}: {}", index, info.completed[index]);
                }
            }
            Some("add") => {
                match processed_command.next() {
                    Some(value) => {
                        let mut name = vec![value];
                        name.append(&mut processed_command.collect::<Vec<&str>>());
                        let name = name.join(" ");

                        println!("Creating todo: {}", name);
                        println!("Enter a description for {}", name);
                        let mut desc = String::new();
                        fix_input(&mut desc);
                        info.uncompleted.push(TodoItem::new(String::from(name), desc))
                    },
                    None => {
                        println!("A name must be supplied when adding a todo.\n\
                        Type \"help\" for more information.");
                    }
                }
            },
            Some("delete") => {
                match processed_command.next() {
                    Some("uncompleted") => {
                        match processed_command.next() {
                            Some(value) => {
                                let value = value.parse::<usize>();
                                match value {
                                    Ok(index) => {},
                                    Err(_) => {
                                        println!("The index must be a number.");
                                    },
                                }
                            },
                            None => {
                                println!("The index of the todo to delete must be specified.");
                            },
                        }
                    },
                    Some("completed") => {

                    },
                    Some(value) => {
                        println!("Invalid branch: {}", value);
                    }
                    None => {
                        println!("You must specify which branch to delete from:\n\
                        completed or uncompleted.");
                    },
                }
            },
            Some("help") => {
                println!("Helping");
            },
            None => {
                println!("You must enter a command.\
                Type \"exit\" to exit or \"help\" for a list of commands.");
            }
            _ => {
                println!("Unrecognized command: {}", command);
            },
        }

        fix_input(&mut command);
    }
    println!("Exiting.");
}
