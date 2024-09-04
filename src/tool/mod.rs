pub mod tool {
    pub struct Tool {
        pub name: String,
        pub command: String,
        pub args: Vec<String>
    }

    impl Tool {
        pub fn build(args: &[String]) -> Result<Tool, &'static str> {
            let name = args[2].clone();
            let command = args[3].clone();
            let mut tool_args: Vec<String> = Vec::new();

            if args.len() > 3 {
                for item in args[4..].into_iter() {
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
}