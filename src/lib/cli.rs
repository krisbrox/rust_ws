pub struct Cli {
    pub args: Vec<String>
}

impl Cli {
    pub fn from(args: Vec<String>) -> Result<Self, String> {
        let arg0 = args[0].as_str();
        if args.len() <= 1 {
            return Err(Self::usage(arg0));
        }

        let val = Cli {
            args
        };

        Ok(val)
    }
    fn usage(arg0: &str) -> String {
        format!("usage:\n{} <args>", arg0)
    }
}
