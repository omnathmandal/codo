use clap::Parser;
use colored::Colorize;
use dotenv::dotenv;
use rusqlite::Connection;
use std::env;
use std::process::exit;

mod cli;
mod crud;

use cli::{Cli, Commands};
use crud::Crud;

fn main() {
    dotenv().ok();
    let url = match env::var("DATABASE_URL") {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error Occured! {}", e);
            exit(1);
        }
    };

    let connection = match Connection::open(url) {
        Ok(con) => con,
        Err(e) => {
            eprintln!("Error Occured! {}", e);
            exit(1);
        }
    };

    let todo = Crud { conn: connection };

    let cli = Cli::parse();

    match todo.new() {
        Ok(_) => (),
        Err(_) => {
            eprintln!("{}", "Failed to create database!".bright_red());
            exit(1);
        }
    }

    match &cli.command {
        Commands::Add { text } => {
            let txt = match text {
                Some(x) => x,
                None => {
                    eprintln!("{}", "Error!".bright_red());
                    std::process::exit(1);
                }
            };
            match todo.add(&txt) {
                Ok(_) => println!("{}", "Todo created!".bright_green()),
                Err(_) => {
                    eprintln!("{}", "Failed to add todo!".bright_red());
                    exit(1);
                }
            }
        }
        Commands::Done { index } => {
            let idx = match index {
                Some(x) => x,
                None => {
                    eprintln!("Error!");
                    exit(1);
                }
            };

            match todo.done(idx) {
                Ok(_) => println!(
                    "{}",
                    "Todo checked off!".bright_green().bold()
                ),
                Err(_) => {
                    eprintln!("{}", "Failed to check off todo!".bright_red());
                    exit(1);
                }
            }
        }
        Commands::Remove { index } => {
            let idx = match index {
                Some(x) => x,
                None => {
                    eprintln!("{}", "Error!".bright_red());
                    exit(1);
                }
            };

            match todo.delete(idx) {
                Ok(_) => println!(
                    "{}",
                    "Todo deleted!".bright_red().bold()
                ),
                Err(_) => {
                    eprintln!("{}", "Failed to delete todo!".bright_red());
                    exit(1);
                }
            }
        }
        Commands::Change { index, new } => {
            let idx = match index {
                Some(x) => x,
                None => {
                    eprintln!("{}", "Error!".bright_red());
                    exit(1);
                }
            };

            let new = match new {
                Some(x) => x,
                None => {
                    eprintln!("{}", "Error!".bright_red());
                    exit(1);
                }
            };

            match todo.modify(idx, &new) {
                Ok(_) => println!(
                    "{}",
                    "Todo modified!".bright_green().bold()
                ),
                Err(_) => {
                    eprintln!("{}", "Failed to modify todo!".bright_red());
                    exit(1);
                }
            }
        }
        Commands::Show => match todo.show() {
            Ok(_) => (),
            Err(_) => {
                eprintln!("{}", "Failed to display all todos!".bright_red());
                exit(1);
            }
        },
        Commands::Clear => match todo.drop() {
            Ok(_) => println!(
                "{}",
                "Cleared all todos!".bright_red().bold()
            ),
            Err(_) => {
                eprintln!("{}", "Failed to delete all todo!".bright_red());
                exit(1);
            }
        },
        Commands::Inc => match todo.incomplete() {
            Ok(_) => (),
            Err(_) => {
                eprintln!(
                    "{}",
                    "Failed to display incomplete todo!".bright_red()
                );
                exit(1);
            }
        },
        Commands::Comp => match todo.complete() {
            Ok(_) => (),
            Err(_) => {
                eprintln!(
                    "{}",
                    "Failed to display completed todo!".bright_red()
                );
                exit(1);
            }
        },
    }
}
