use std::env;
use todo::tool::tool::Tool;
fn main() {
    let args: Vec<String> = env::args().collect();

   let t = Tool::build(&args).unwrap();

    println!("{:?}", t.args);

}
