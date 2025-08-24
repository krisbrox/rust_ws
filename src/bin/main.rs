use calc::calculator::Calculator;
use calc::cli::Cli;

fn main() {
    let mut args: Vec<String> = vec![];
    for arg in std::env::args() {
        args.push(arg);
    }

    let data = match Cli::from(args) {
        Ok(cli) => {
            println!("args: {:?}", cli.args);
            cli
        }
        Err(msg) => {
            println!("{}", msg.as_str());
            return;
        }
    };
    let output = Calculator::calculate(data.args.as_str());
    println!("\nExpression evaluated to: {}\n", output);
}
