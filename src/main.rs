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

fn run_command(mut todos: Vec<Todo>) -> Vec<Todo> {
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
            todos
        }
        "mark" => {
            todo!();
        }
        // TODO: Maybe add filtering?
        "list" => {
            for todo in &todos {
                let completed = if todo.completed { "[X]" } else { "[ ]" };
                println!("{} - {}", completed, todo.name);
            }
            todos
        }
        // TODO: Add remove all (clear)
        "remove" => {
            todo!();
        }
        "quit" => {
            std::process::exit(0);
        }
        _ => {
            println!("Invalid command");
            todos
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

    println!("Todo App");
    println!("========");
    println!(
        "Commands:\n- add <item> [-due <date>]\n- mark <task>\n- list\n- remove <task>\n- quit"
    );
    println!("========");
    println!("Usage:\n <command> <args>");

    loop {
        todos = run_command(todos);
        save_todos(&path, &todos);
    }
}
