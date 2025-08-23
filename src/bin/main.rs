use calc::cli::Cli;
use calc::calculator::Calculator;

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

    let calculator = Calculator::from(&data.args);
    let res = calculator.calculate();
    
    println!("{}", res)
}
