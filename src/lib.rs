use std::fs;
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
    let mut todo_list = tool.args.join(" ");
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

pub fn read_todo(tool: Tool) -> Result<(), &'static str> {
    let file = fs::OpenOptions::new().read(true).open("todo.txt");
    match file {
        Ok(file) => println!("{file}"),
        Err(error) => panic!("Can not read file: {:?}", error.kind())
    };
    Ok(())
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
    let mut task = String::from("");
    for item in tool.args[1..].iter() {
        task = task + " " + item;
    }
    let mut file = fs::read_to_string(tool.args[0].clone()).unwrap();


    Ok(())
}


// ************************** UNIT TESTS ***********************************

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_new_todo() {
        let test_tool = Tool {
            name: String::from("todo"),
            command: String::from("new"),
            args: vec![String::from("test.txt")]
        };
        assert_eq!(new_todo(test_tool).unwrap(), ());
        let remove_file = fs::remove_file("test.txt");

    }
    #[test]
    fn test_read_todo() {
        let mut test_file = fs::File::create_new(("test.txt"));
        fs::write("test.txt", "This is a test").unwrap();

        let test_tool = Tool {
            name: String::from("todo"),
            command: String::from("read"),
            args: vec![String::from("test.txt")]
        };
        assert_eq!(read_todo(test_tool).unwrap(), ());
        let remove_file = fs::remove_file("test.txt");

    }
    #[test]
    fn test_add_todo_item() {
        let mut test_file = fs::File::create_new(("test.txt"));
        let test_tool = Tool {
            name: String::from("todo"),
            command: String::from("add"),
            args: vec![String::from("test.txt"), String::from("test"), String::from("test")]
        };
        let updated_file = add_todo(test_tool).unwrap();
        // let remove_file = fs::remove_file("test.txt");




    }
}









