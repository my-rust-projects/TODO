
/*
TODO: Create function to create a todo file in the user's documents folder
    - Find the location of the user folder
    - cd to Document folder
    - Create a folder called 'Todos'
    - Let the user name the todos file
TODO: Create function to write to Todo file
    - Open the 'Todos' file with the name specified by the user
TODO: Create function to read from Todo file
TODO: Add the function to strikethrough completed Todo item
TODO: Add the ability to create sub-lists
TODO: Add a config.toml file
    - Make the program cross-compile.
 */
use std::env;
use todo::Path;
fn main() {
    let args: Vec<String> = env::args().collect();
    let os = env::consts::OS;

    if os == "linux" {
        let path = Path::linux().unwrap();
        println!("{}",path.path )
    }
}
