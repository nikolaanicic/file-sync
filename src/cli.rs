use std::path::Path;

use clap::Parser;

use crate::dir::{cmp_dirs, sync_dir};

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Args {
    #[arg(short, long)]
    pub source: String,
    #[arg(short, long)]
    pub destination: String,

    #[arg(short, long)]
    pub fake_run: bool,

    #[arg(short, long)]
    pub command: String,

    #[arg(short, long)]
    pub verbose: bool,
}
const POSSIBLE_COMMANDS: [&'static str; 2] = ["cmp", "sync"];

fn command_exists(command: &String) -> bool {
    for &cmd in POSSIBLE_COMMANDS.iter() {
        if cmd == command.as_str() {
            return true;
        }
    }
    false
}

pub fn parse_cli_args() -> Result<Args, String> {
    let args = Args::parse();

    if !command_exists(&args.command) {
        return Err(format!("invalid command {}", args.command));
    }

    log::debug!("parsed: {:?}", args);

    Ok(args)
}

fn execute_cmp_command(args: &Args) -> Result<(), String> {
    match cmp_dirs(&Path::new(&args.source), &Path::new(&args.destination)) {
        Ok(cmp_result) => {
            for (filename, exists) in cmp_result.iter() {
                println!("{:<25} {:<4}", filename, exists)
            }

            Ok(())
        }
        Err(e) => Err(format!("can't execute cmp command: {}", e)),
    }
}

fn execute_sync_command(args: &Args) -> Result<(), String> {
    match sync_dir(&Path::new(&args.source), &Path::new(&args.destination)) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("can't execute sync command: {}", e)),
    }
}

pub fn execute_command(args: &Args) -> Result<(), String> {
    match args.command.as_str() {
        "cmp" => execute_cmp_command(args),
        "sync" => execute_sync_command(args),
        _ => Err("unkown command".to_string()),
    }
}
