use std::env;
use todo::tool::tool::Tool;
use todo;
fn main() {
    todo::check_os().unwrap();

    let args: Vec<String> = env::args().collect();
    let tool = Tool::build(&args).unwrap();

    todo::run(tool).unwrap();
}
