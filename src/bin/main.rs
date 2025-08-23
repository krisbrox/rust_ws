use calc::calculator::Calculator;
use calc::cli::Cli;
use calc::expression::Expression;

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

    let expression_res = Expression::from(data.args.as_str());
    match expression_res {
        Ok(expr) => {
            let output = Calculator::calculate(expr);
            println!("{}", output);
        }
        Err(msg) => {
            println!("{}", msg)
        }
    }
}
