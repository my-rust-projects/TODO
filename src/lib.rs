use std::fs;
use std::error::Error;

pub struct Tool {
    name: String,
    pub command: String,
    pub args: Vec<String>
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

pub fn run(tool: Tool) -> Result<(),()> {

}