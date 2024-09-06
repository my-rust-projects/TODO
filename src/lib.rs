use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use crate::tool::tool::Tool;
// TODO: Make files into a flat database structure. Use split(), and collect()
/*
Command structure -> to-do(app) daily(todolist name) [new, read, add, remove](command)
                        take out the trash(todoItem)
                1 - 2 - 3 - 4
*/

pub mod tool;
pub mod config;

pub fn new_todo(tool: Tool) -> Result<(), &'static str> {
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

pub fn read_todo(tool: Tool) -> Result<fs::File, &'static str> {
    let file = fs::OpenOptions::new().read(true).open("todo.txt");
    let return_file = match file {
        Ok(file) => file,
        Err(error) => panic!("Can not read file: {:?}", error.kind())
    };
    Ok(return_file)
}

pub fn add_todo(tool: Tool) -> Result<(), &'static str> {
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

pub fn remove_todo(tool: Tool) -> Result<(), &'static str> {
    let mut task = tool.args.join(" ");
    task = task + "::";
    let mut file = fs::read_to_string("todo.txt").unwrap();
    file = file.replace(task.as_str(), "");
    let updated_file = OpenOptions::new().write(true).open("todo.txt");
    match updated_file {
        Ok(mut file) => file.write(task.as_bytes()).unwrap(),
        Err(error) => panic!("Can't write to file: {:}", error.kind())
    };


    Ok(())
}



// ************************** UNIT TESTS ***********************************

#[cfg(test)]
mod tests {
    use std::io::Read;
    use super::*;

    #[test]
    fn set_up() {
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
        let updated_file = add_todo(test_tool).unwrap();
    }

    #[test]
    fn test_3_read_todo() {
        let test_tool = Tool {
            name: String::from("Daily"),
            command: String::from("read"),
            args: vec![]
        };
        let info = fs::read_to_string("todo.txt").unwrap();

        assert_eq!(info, "Daily::test test::");
        fs::remove_file("todo.txt").unwrap();

    }
}









