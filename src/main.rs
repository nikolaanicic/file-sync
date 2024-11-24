use cli::execute_command;
use env_logger;

mod cli;
mod dir;

fn main() {
    let args = cli::parse_cli_args().unwrap();

    if args.verbose {
        env_logger::init();
    }

    match execute_command(&args) {
        Err(e) => println!("{}", e),
        _ => return,
    }
}
