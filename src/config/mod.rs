

pub mod Config {
    use std::fs;
    use std::env;

    pub struct Config {
        pub os: String,
        pub path: String
    }

    impl Config {
        pub fn linux() -> Result<Config, &'static str> {
            let path = "/opt/todo";
            let set_dir  = fs::create_dir(&path);
            let working_dir = env::set_current_dir((&path));


            Ok(Config {
                os: String::from("linux"),
                path: String::from(path)
            })
        }

        pub fn win() -> Result<Config, &'static str> {
            let path = "C:\\Program Files\\todo";
            let set_dir = fs::create_dir(&path);
            let working_dir = env::set_current_dir(&path);

            Ok(Config {
                os: String::from("windows"),
                path: String::from(path)
            })
        }
    }
}
