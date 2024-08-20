use std::fs;
use std::error::Error;
use std::env::consts::OS;

pub struct Tool {
    name: String,
    pub command: String,
    pub args: Vec<String>
}

pub struct Path {
    os: String,
    path: String
}


// run
// ----------------
// init - windows/linux/mac
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
            for item in args[3..] {
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
    pub fn linux() -> Result<Path, ()> {
        let dir = fs::create_dir("/opt/todo/");

        Ok(Path {
            os: String::from("linux"),
            path: String::from("/opt/todo/")
        })
    }
}



// ************************** UNIT TESTS ***********************************

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linux() {
        let test = OS;
        assert_eq!(test, "linux");
    }
}









