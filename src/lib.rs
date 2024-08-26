use std::{fs, env, path};
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

pub fn new_todo(name: path) -> Result<(), &'static str> {
    let file_result = fs::File::create_new(name);
    match file_result {
        Ok(())| Err(Error) => panic!("Can not create file: {Error:?}")
    }
}



// ************************** UNIT TESTS ***********************************

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_todo() {

    }
}









