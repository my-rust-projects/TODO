use std::{env, fs};

pub mod Path {
    pub struct Path {
        pub os: String,
        pub path: String
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
}
