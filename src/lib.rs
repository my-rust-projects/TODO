use std::{fs, env, path};
use std::io::Error;
use crate::tool::Tool::Tool;
use std::ffi::{OsStr, OsString};

// run
// ----------------
// new
// open
// save
// add
// remove
pub mod tool;
pub mod config;

pub fn new_todo(tool: Tool) -> Result<(), &'static str> {
    let file_result = fs::File::create_new(tool.args[0].clone());
    let file = match file_result {
        Ok(file) => file,
        Err(error) => panic!("Can not create file: {:?}", error.kind())
    };

    Ok(())
}



// ************************** UNIT TESTS ***********************************

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_todo() {
        let test_tool = Tool {
            name: String::from("todo"),
            command: String::from("new"),
            args: vec![String::from("test.txt")]
        };
        assert_eq!(new_todo(test_tool).unwrap(), ());
        let remove_file = fs::remove_file("test.txt");

    }
}









