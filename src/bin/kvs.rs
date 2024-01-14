use std::io::Write;

use clap::{arg, command, Command};
use kvs::{KvStore, KvsError, Result};

fn main() -> Result<()> {
    let mut store = KvStore::new();

    loop {
        let line = readline()?;
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        match main_loop(line, &mut store) {
            Ok(quit) => {
                if quit {
                    break;
                }
            }
            Err(err) => {
                write!(std::io::stdout(), "{err}")?;
                std::io::stdout().flush()?;
            }
        }
    }

    Ok(())
}

fn readline() -> Result<String> {
    write!(std::io::stdout(), "> ")?;
    std::io::stdout().flush()?;

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;

    Ok(buffer)
}

fn main_loop(line: &str, store: &mut KvStore) -> Result<bool> {
    let args =
        shlex::split(line).ok_or(KvsError::UnexpectedCommandType)?;

    let matches = cli().try_get_matches_from(args)?;

    match matches.subcommand() {
        Some(("get", sub_matches)) => {
            let key = sub_matches.get_one::<String>("KEY").unwrap();
            let value = store.get(key.clone())?;

            match value {
                Some(_v) => println!("{{ key: {:?}, value: {:?}}}", key, _v),
                None => println!("key {:?} not found", key),
            }

            Ok(false)
        }
        Some(("set", sub_matches)) => {
            let key = sub_matches.get_one::<String>("KEY").unwrap();
            let value = sub_matches.get_one::<String>("VALUE").unwrap();

            store.set(key.clone(), value.clone())?;

            println!("{{ key: {:?}, value: {:?}}}", key, value);

            Ok(false)
        }
        Some(("rm", sub_matches)) => {
            let key = sub_matches.get_one::<String>("KEY").unwrap();
            store.remove(key.clone())?;
            
            println!("key: {:?} was removed", key);

            Ok(false)
        }
        Some(("start", _matches)) => Ok(false),
        Some(("quit", _matches)) => {
            println!("Exiting...");

            Ok(true)
        }
        Some((name, _matches)) => unimplemented!("{name}"),
        _ => unreachable!("subcommand required"),
    }
}

fn cli() -> Command {
    command!()
        .multicall(true)
        .propagate_version(true)
        .subcommand_required(true)
        .subcommand(
            Command::new("set")
                .about("Sets a value at a new given key")
                .arg(arg!(<KEY> "The new key"))
                .arg_required_else_help(true)
                .arg(arg!(<VALUE> "The value for the given key"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("get")
                .about("Gets the value at a given key")
                .arg(arg!(<KEY> "The key to a given value"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("rm")
                .about("Removes the key value pair for a given key")
                .arg(arg!(<KEY> "The key to be removed"))
                .arg_required_else_help(true),
        )
        .subcommand(Command::new("start").about("Start"))
        .subcommand(Command::new("quit").alias("exit").about("Quit"))
}
