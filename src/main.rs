use chrono::DateTime;
use serde::Deserialize;
use serde::Serialize;
use std::io::Write;

#[derive(Deserialize, Serialize)]
struct Todo {
    name: String,
    completed: bool,
    _created: DateTime<chrono::Utc>,
    _due: Option<DateTime<chrono::Utc>>,
}

fn load_todos(path: &std::path::Path) -> Vec<Todo> {
    let file = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);
    let todos = serde_json::from_reader(reader).unwrap();
    todos
}

fn save_todos(path: &std::path::Path, todos: &Vec<Todo>) {
    let file = std::fs::File::create(path).unwrap();
    serde_json::to_writer(file, &todos).unwrap();
}

fn run_command(todos: &mut Vec<Todo>) -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    let mut input = input.split_whitespace();
    let command = input.next().unwrap();
    let args: Vec<&str> = input.collect();

    match command {
        // TODO: Need to incorporate the due date
        "add" => {
            let name = args.join(" ");
            let todo = Todo {
                name,
                completed: false,
                _created: chrono::Utc::now(),
                _due: None,
            };
            todos.push(todo);
            println!("Added");
            1
        }
        "complete" | "mark" => {
            let index = args.join(" ");
            if let Ok(index) = index.parse::<usize>() {
                if let Some(todo) = todos.get_mut(index - 1) {
                    todo.completed = true;
                    println!("Done");
                } else {
                    println!("Todo {} not found", index);
                }
            } else {
                println!("Invalid index");
            }
            1
        }
        // TODO: Maybe add filtering?
        "list" | "ls" => {
            for (index, todo) in todos.iter().enumerate() {
                let completed = if todo.completed { "[X]" } else { "[ ]" };
                println!("{}. {} - {}", index + 1, completed, todo.name);
            }
            1
        }
        // TODO: Add remove all (clear)
        "remove" => {
            let index = args.join(" ");
            if index == "all" || index == "a" {
                todos.clear();
                println!("Cleared");
                return 1;
            } else if let Ok(index) = index.parse::<usize>() {
                if index < todos.len() {
                    todos.remove(index - 1);
                    println!("Todo {} has been deleted", index);
                } else {
                    println!("Todo {} not found", index + 1);
                }
            } else {
                println!("Invalid index");
            }
            1
        }
        "quit" | "save" => 0,
        "help" => {
            println!("========");
            println!(
                "Commands:\n- add <item> [-due <date>]\n- complete <task>\n- list\n- remove <task>\n- quit\n- help"
            );
            println!("========");
            println!("Usage:\n <command> <args>");
            println!("========");
            1
        }
        _ => {
            println!("Invalid command");
            1
        }
    }
}

fn main() {
    let path = std::path::Path::new("./todos.json");
    if !path.exists() {
        let mut file = std::fs::File::create(path).unwrap();
        file.write_all(b"[]").unwrap();
    }

    let mut todos = load_todos(&path);

    println!("========");
    println!("Todo App");
    println!("========");
    println!(
        "Commands:\n- add <item> [-due <date>]\n- complete <task>\n- list\n- remove <task>\n- quit\n- help"
    );
    println!("========");
    println!("Usage:\n <command> <args>");
    println!("========");

    loop {
        let output = run_command(&mut todos);
        if output == 0 {
            break;
        }
    }

    save_todos(path, &todos);
    std::process::exit(0);
}
