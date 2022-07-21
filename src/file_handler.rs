use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use termion::{ color };
use crate::cli::Priority;
use comfy_table::*;
use comfy_table::presets::UTF8_FULL;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;


fn init_todo_file() -> std::io::Result<()> {
    File::create(".todo")?;
    Ok(())
    
}

pub struct Todo {
    pub todo: String,
    pub priority: Priority,
    pub created_on: String
}

pub fn read_todo_file() -> std::io::Result<()> {
    if !(Path::new(".todo").exists()) {
        init_todo_file()?;
    }

    let mut file = File::open(".todo")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Id").add_attribute(Attribute::Bold).fg(Color::Cyan).set_alignment(CellAlignment::Center), 
            Cell::new("Todo").add_attribute(Attribute::Bold).fg(Color::Cyan),
            Cell::new("Priority").add_attribute(Attribute::Bold).fg(Color::Cyan),  
            Cell::new("Created on").add_attribute(Attribute::Bold).fg(Color::Cyan), 
        ]);

    if contents.is_empty() {
        println!("{}No todos today; Yay..!", color::Fg(color::LightGreen))
    } else{
        for (id, line) in contents.lines().enumerate() {
            let content_split: Vec<&str> = line.split("|").collect();
            let id = id+1;
            let mut color: Color = Color::Grey;
            match content_split[1] {
                "high" => color = Color::Red,
                "medium" => color = Color::Yellow,
                "low" => color = Color::Green,
                _ => {}
            }
            table.add_row(vec![
                Cell::new(id.to_string()).fg(color),
                Cell::new(content_split[0].to_string()).fg(color),
                Cell::new(content_split[1].to_string()).fg(color),
                Cell::new(content_split[2].to_string()).fg(color),
            ]);
        }
        println!("{}", table)
    }
    
    Ok(())
}

pub fn add_todo(todo: &Todo) -> std::io::Result<()> {
    if !(Path::new(".todo").exists()) {
        init_todo_file()?;
    }
    let mut file = OpenOptions::new().append(true).open(".todo")?;
    let mut todo_string = String::new();
    let raw = format!("{}|{}|{}", todo.todo, todo.priority.as_str(), todo.created_on);
    todo_string.push_str(&raw);
    todo_string.push_str("\n");
    file.write(todo_string.as_bytes())?;

    println!("{}Successfully added {}", color::Fg(color::LightGreen), todo.todo);

    Ok(())
}
