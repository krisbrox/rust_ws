
fn main() {
    let mut args: Vec<String> = vec![];
    for arg in std::env::args() {
        args.push(arg);
    }
    
    let cli = cli::Cli::from(args);
}


pub mod cli {
    pub struct Cli;

    impl Cli {
        pub fn from(args: Vec<String>) {
            let arg0 = args[0].as_str();
            if args.len() <= 1 {
                usage(arg0);
                return
            }

            println!("args: {:?}", args)
        }
    }    
    fn usage(arg0: &str) {
        println!("usage:");
        println!("{} <args>", arg0);
    }
}
