

pub mod config {
    use std::fs;
    use std::env;

    pub struct Config {
        pub os: String,
        pub path: String
    }

    impl Config {
        pub fn linux() -> Result<Config, &'static str> {
            let path = "/opt/todo";
            if !fs::metadata(&path).is_ok() {
                fs::create_dir(&path).expect("Directory Exists");
                env::set_current_dir(&path).expect("Can not change Directory");
                fs::File::create_new("todo.txt").expect("File Exists");
            } else {
                env::set_current_dir(&path).expect("Can not change Directory");
            }

            Ok(Config {
                os: String::from("linux"),
                path: String::from(path)
            })
        }

        pub fn win() -> Result<Config, &'static str> {
            let path = "C:\\Program Files\\todo";
            if !fs::metadata(&path).is_ok() {
                fs::create_dir(&path).expect("Directory Exists");
                env::set_current_dir(&path).expect("Can not change Directory");
                fs::File::create_new("todo.txt").expect("File Exists");
            }
            Ok(Config {
                os: String::from("windows"),
                path: String::from(path)
            })
        }
    }
}
