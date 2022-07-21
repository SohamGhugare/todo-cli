use clap::{ Parser, Subcommand, Args };
use std::error::Error;
use crate::file_handler;
use crate::file_handler::Todo;
use chrono::prelude::*;

#[derive(Parser)]
#[clap(about="Minimal Todo app", version="0.1.0", author="SohamGhugare")]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands
}

#[derive(Args)]
struct Add {
    todo: String,
    #[clap(short, long)]
    priority: Option<String>
}

impl Add {
    fn new(todo: String, priority: Priority) -> Result<(), Box<dyn Error>> {
        let todo = Todo {
            todo: todo,
            priority: priority,
            created_on: Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
        };
        file_handler::add_todo(&todo)?;
        Ok(())
    }
}

#[derive(Subcommand)]
enum Commands {
    Add(Add),
    List,
}

pub enum Priority {
    High,
    Medium,
    Low
}

impl Priority {
    fn new(priority: &String) -> Result<Priority, &str> {
        match priority.to_lowercase().as_str() {
            "high" => Ok(Priority::High),
            "med" => Ok(Priority::Medium),
            "low" => Ok(Priority::Low),
            _ => Err("Invalid priority.")
        }
        
    }

    pub fn as_str(&self) -> &str {
        match &self {
            Priority::High => "high",
            Priority::Low => "low",
            Priority::Medium => "medium"
        }
    }
}

fn list_todos() -> Result<(), Box<dyn Error>>{
    file_handler::read_todo_file()?;
    Ok(())
}

pub fn parse_cli() -> Result<(), Box<dyn Error>>{
    let args = Cli::parse();

    match &args.command {
        Commands::Add(Add {todo, priority}) => {
            if let Some(v) = priority {
                Add::new(todo.clone(), Priority::new(&v)?)?;
            } else {
                Add::new(todo.clone(), Priority::new(&"low".to_string())?)?;
            }
        },
        Commands::List => list_todos()?
    }

    Ok(())
}