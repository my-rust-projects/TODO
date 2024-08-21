use std::fs;
use std::error::Error;
use std::env;

pub struct Tool {
    name: String,
    pub command: String,
    pub args: Vec<String>
}

pub struct Path {
    pub os: String,
    pub path: String
}


// run
// ----------------
// new
// open
// save
// add
// remove


impl Tool {
    pub fn build(args: &[String]) -> Result<Tool, &'static str> {
        let name = args[1].clone();
        let command = args[2].clone();
        let mut tool_args: Vec<String> = Vec::new();

        if args.len() > 3 {
            for item in args[3..].into_iter() {
                tool_args.push(item.clone());
            }
        }

        Ok(Tool {
            name,
            command,
            args: tool_args
        })
    }
}


impl Path {
    pub fn linux() -> Result<Path, &'static str> {
        let path = "/opt/todo";
        let set_dir  = fs::create_dir(&path);
        let working_dir = env::set_current_dir((&path));

        Ok(Path {
            os: String::from("linux"),
            path: String::from(path)
        })
    }

    pub fn win() -> Result<Path, &'static str> {
        let path = "C:\\Program Files\\todo";
        let set_dir = fs::create_dir(&path);
        let working_dir = env::set_current_dir(&path);

        Ok(Path {
            os: String::from("windows"),
            path: String::from(path)
        })
    }
}



// ************************** UNIT TESTS ***********************************

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linux() {
        let test = env::consts::OS;
        assert_eq!(test, "linux");
    }
}









