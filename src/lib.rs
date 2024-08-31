use std::fs;
use crate::tool::tool::Tool;

// run
// ----------------
// remove
pub mod tool;
pub mod config;

pub fn new_todo(tool: Tool) -> Result<(), &'static str> {
    let file_result = fs::File::create_new(tool.args[0].clone());
    match file_result {
        Ok(file) => file,
        Err(error) => panic!("Can not create file: {:?}", error.kind())
    };

    Ok(())
}

pub fn read_todo(tool: Tool) -> Result<(), &'static str> {
    let file_result = fs::read_to_string(tool.args[0].clone());
    match file_result {
        Ok(file) => println!("{file}"),
        Err(error) => panic!("Can not read file: {:?}", error.kind())
    };
    Ok(())
}

pub fn add_todo(tool: Tool) -> Result<(), &'static str> {
    let mut task = String::from("");
    for item in tool.args[1..].iter() {
        task = task + " " + item;
    }
    task = task + " ";

    let mut file = fs::read_to_string(tool.args[0].clone()).unwrap();
    file = file + &task;
    fs::write(tool.args[0].clone(), file.clone()).expect("Can not write to file");
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









