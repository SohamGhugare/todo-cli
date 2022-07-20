use clap::{ Parser, Subcommand, Args };
use std::error::Error;

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
    fn new(todo: &String, priority: Priority) -> Result<(), Box<dyn Error>> {
        add_todo(&todo, priority)?;
        Ok(())
    }
}

#[derive(Subcommand)]
enum Commands {
    Add(Add),
    List,
}

enum Priority {
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
}

fn add_todo(todo: &String, priority: Priority) -> Result<(), Box<dyn Error>> {
    match priority {
        Priority::High => {
            println!("Adding {} to the list with High priority...", &todo)
        },
        Priority::Medium => {
            println!("Adding {} to the list with Medium priority...", &todo)
        },
        Priority::Low => {
            println!("Adding {} to the list with Low priority...", &todo)
        }
    }


    Ok(())
}

pub fn parse_cli() -> Result<(), Box<dyn Error>>{
    let args = Cli::parse();

    match &args.command {
        Commands::Add(Add {todo, priority}) => {
            if let Some(v) = priority {
                Add::new(todo, Priority::new(&v)?)?;
            } else {
                println!("Adding {} to list...", todo)
            }
        },
        Commands::List => println!("Listing")
    }

    Ok(())
}