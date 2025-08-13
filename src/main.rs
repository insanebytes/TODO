use clap::{Parser, Subcommand};
use colored::Colorize;
use jiff::{Zoned, civil::DateTime};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Write},
    fs,
    io::ErrorKind,
};
use tabled::{Table, Tabled, settings::Style};

#[derive(Parser)]
#[command(version,about="ARG todo task manager",long_about=None,arg_required_else_help=true)]
struct Arguments {
    #[command(subcommand)]
    option: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Adds a new task", arg_required_else_help = true,short_flag='a')]
    Add {
        #[arg(help = "Task description", value_name = "TASK DESCRIPTION")]
        text: String,
        #[arg(help = "Task date, optional if not actual time", value_name = "TASK DATE")]
        date: Option<String>,
    },
    #[command(about = "List all tasks",short_flag='l')]
    List,
    #[command(about = "Marks task as done", arg_required_else_help = true,short_flag='d')]
    Done {
        #[arg(help = "Task id", value_name = "TASK ID")]
        id: u32,
    },
    #[command(about = "Cleans the task database",short_flag='c')]
    Clean,
}

#[derive(Serialize, Deserialize, Debug, Tabled)]
struct Task {
    #[tabled(rename = "Id")]
    id: u32,
    #[tabled(rename = "Description")]
    text: String,
    #[tabled(rename = "Date", display = "display_date_table")]
    date: DateTime,
    #[tabled(rename = "Done")]
    done: bool,
}

fn display_date_table(fecha: &DateTime) -> String {
    let mut fecha_str: String = String::new();
    write!(
        &mut fecha_str,
        "{:0>4}-{:0>2}-{:0>2} {:0>2}:{:0>2}:{:0>2}",
        fecha.year(),
        fecha.month(),
        fecha.day(),
        fecha.hour(),
        fecha.minute(),
        fecha.second()
    )
    .ok();
    return fecha_str;
}

fn main() {
    let args = Arguments::parse();

    match args.option {
        Commands::Add { text, date } => add_task(text, date),
        Commands::List => list_tasks(),
        Commands::Done { id } => mark_task_done(id),
        Commands::Clean => clean_tasks(),
    }
}

fn load_tasks() -> Vec<Task> {
    let tasks: Vec<Task>;

    match fs::read_to_string("task.json") {
        Ok(content) => tasks = serde_json::from_str(&content).ok().unwrap_or_default(),
        Err(error) => {
            if error.kind() != ErrorKind::NotFound {
                eprintln!(
                    "{} {}",
                    "Error loading task".red(),
                    error.to_string().red().italic()
                );
            }
            tasks = Vec::new();
        }
    }

    return tasks;
}

fn save_tasks(tasks: &Vec<Task>) -> Result<(), ()> {
    let json_content =
        serde_json::to_string_pretty(tasks).expect(&"error serializing tasks".red().bold());

    match fs::write("task.json", json_content) {
        Ok(()) => Ok(()),
        Err(error) => {
            eprintln!(
                "{} {}",
                "Error saving tasks".red(),
                error.to_string().red().italic()
            );
            Err(())
        }
    }
}

fn add_task(text: String, date: Option<String>) {
    let mut tasks = load_tasks();
    let id: u32 = (tasks.len() as u32) + 1;

    let date_work: DateTime;

    match date {
        Some(d) => date_work = DateTime::strptime("%F %T", d).unwrap(),
        None => date_work = Zoned::now().datetime(),
    }

    let tarea: Task = Task {
        id: id,
        text: text,
        date: date_work,
        done: false,
    };

    tasks.push(tarea);
    match save_tasks(&tasks) {
        Ok(()) => println!("{}", "Task added succesfully".green().bold()),
        _ => (),
    }
}

fn list_tasks() {
    let tasks = load_tasks();

    if tasks.len() > 0 {
        let mut table = Table::new(tasks);
        table.with(Style::modern());
        println!("{}", table);
    } else {
        println!("{}", "No tasks found".green().bold())
    }
}

fn mark_task_done(id: u32) {
    let mut tasks = load_tasks();

    if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
        task.done = true;

        match save_tasks(&tasks) {
            Ok(()) => println!(
                "{} {} {}",
                "Task".green().bold(),
                id.to_string().green().bold(),
                "marked as done succesfully".green().bold()
            ),
            _ => (),
        }
    } else {
        eprintln!("{}", "Task not found".red())
    }
}

fn clean_tasks() {
    match fs::remove_file("task.json") {
        Ok(()) => println!("{}", "Tasks cleaned sucessfully".green().bold()),
        Err(error) => {
            if error.kind() != ErrorKind::NotFound {
                eprintln!(
                    "{} {}",
                    "Error cleaning task".red(),
                    error.to_string().red().italic()
                );
            } else {
                println!("{}", "Tasks cleaned sucessfully".green().bold())
            }
        }
    }
}
