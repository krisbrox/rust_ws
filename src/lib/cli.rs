pub struct Cli {
    pub args: String
}

impl Cli {
    pub fn from(args: Vec<String>) -> Result<Self, String> {
        let arg0 = args[0].as_str();

        if args.len() != 2 {
            return Err(Self::usage(arg0));
        }

        let args = &args[1..];
        let arg = args[0].as_str();

        let val = Cli {
            args: arg.to_string()
        };

        Ok(val)
    }

    fn usage(arg0: &str) -> String {
        format!(
            "usage: this calculator should be called with a single string argument.\n\
            The argument should be a string containing the expression you want to calculate.\n\
            The argument string should be in a supported syntactic format,\n\
            e.g. infix \"(2 + 3)\", polish \"+ 2 3\" etc.\n\n\
            example: \n   $ {} \"+ 3 4\" \n-> $ 7", arg0
        )
    }
}
