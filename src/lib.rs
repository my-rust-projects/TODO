use std::{env, fs};
use std::io::Write;
use crate::config::config::Config;
use crate::tool::tool::Tool;

pub mod tool;
pub mod config;

fn new_todo(tool: Tool) -> Result<(), &'static str> {
    // Gets the to do list name and adds :: to the end of it
    // before converting the string to bytes.
    let mut todo_list = tool.name.clone();
    todo_list = todo_list + "::";
    let byt_list = todo_list.as_bytes();

    // Gets the file and opens it then adds the to do list name to file.
    let file = fs::OpenOptions::new().append(true).open("todo.txt");
    match file {
        Ok(mut file) => file.write(byt_list).unwrap(),
        Err(error) => panic!("Can not create todo: {:?}", error.kind())
    };

    Ok(())
}

fn read_todo() -> Result<(), &'static str> {
    let file = fs::OpenOptions::new().read(true).open("todo.txt");
    match file {
        Ok(file) => file,
        Err(error) => panic!("Can not read file: {:?}", error.kind())
    };
    let str_file = fs::read_to_string("todo.txt").unwrap();
    let split_file = str_file.split("::");
    for word in split_file {
        println!("{:?}", word);
    }
    Ok(())
}

fn add_todo(tool: Tool) -> Result<(), &'static str> {
    let mut task = tool.args.join(" ");
    task = task + "::";
    let byte_task = task.as_bytes();

    let mut file = fs::OpenOptions::new()
        .append(true)
        .open("todo.txt")
        .unwrap();

    file.write(byte_task).expect("Can not write to file.");

    Ok(())
}

fn remove_todo(tool: Tool) -> Result<(), &'static str> {
    let mut task = tool.args.join(" ");
    task = task + "::";
    let file = fs::read_to_string("todo.txt").unwrap();
    let new_file = file.replace(task.as_str(), "");
    let byte_file = new_file.as_bytes();
    fs::write("todo.txt", byte_file).expect("Can't write to file.");


    Ok(())
}

pub fn check_os() -> Result<Config, &'static str> {
    let os  = env::consts::OS;
    if os == "linux" {
        let config = Config::linux().unwrap();
        Ok(config)
    } else if os == "windows" {
        let config = Config::win().unwrap();
        Ok(config)
    } else {
        panic!("Unsupported OS!");
    }
}

pub fn run(tool: Tool) -> Result<(), &'static str> {
    let command = tool.command.clone();
    if command == "new" {
        new_todo(tool).expect("Error");
    } else if command == "read" {
        read_todo().expect("Error");
    } else if command == "add" {
        add_todo(tool).expect("Error");
    } else if command == "remove" {
        remove_todo(tool).expect("Error");
    } else {
        panic!("Unknown command: {}", command);
    }

    Ok(())
}



// ************************** UNIT TESTS ***********************************

#[cfg(test)]
mod tests {
    use std::env;
    use super::*;

    #[test]
    fn set_up() {
        let curr_dir = env::current_dir().unwrap();
        let dir = fs::read_dir(curr_dir).unwrap();
        for file in dir {
            if file.unwrap().file_name() == "todo.txt" {
                fs::remove_file("todo.txt").unwrap();
            }
        }
        fs::File::create_new("todo.txt").unwrap();
    }


    #[test]
    fn test_1_new_todo() {
        let test_tool = Tool {
            name: String::from("Daily"),
            command: String::from("new"),
            args: vec![]
        };
        assert_eq!(new_todo(test_tool).unwrap(), ());
    }

    #[test]
    fn test_2_add_todo_item() {
        let test_tool = Tool {
            name: String::from("Daily"),
            command: String::from("add"),
            args: vec![String::from("test"), String::from("test")]
        };
        add_todo(test_tool).unwrap();
    }

    #[test]
    fn test_3_read_todo() {
        let info = fs::read_to_string("todo.txt").unwrap();
        assert_eq!(info, "Daily::test test::");
    }

    #[test]
    // #[ignore]
    fn test_4_remove_todo() {
        let test_tool = Tool {
            name: String::from("Daily"),
            command: String::from("remove"),
            args: vec![String::from("test"), String::from("test")]
        };
        remove_todo(test_tool).unwrap();
        let info = fs::read_to_string("todo.txt").unwrap();
        fs::remove_file("todo.txt").unwrap();

        assert_eq!(info, "Daily::")
    }
}









