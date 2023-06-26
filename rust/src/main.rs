use chrono::Local;
use std::env;
use std::io::{self, BufRead};


#[allow(dead_code)]
#[derive(Debug)]
struct Task {
    name: String,
    description: String,
    is_completed: bool,
    time_created: chrono::DateTime<chrono::Local>,
    date_due: String
}

fn main() -> () {
    let mut task_list: Vec<Task> = vec![
        Task {
            name: String::from("Alice"),
            description: String::from("Alice"),
            is_completed: false,
            time_created: Local::now(),
            date_due: String::from("Alice"),
        },
        ];
    

    parse_commands(&mut task_list);
    println!("{:?}", task_list);
}


fn parse_commands(tasks: &mut Vec<Task>) -> () {
    let args: Vec<String> = env::args().collect();
    
    match args.len() {
        2 => match args[1].as_str() {
                "add" => assign_task(tasks, /* String::from("ToDo App"), String::from("Build ToDo app"), String::from("11/22/24") */),
                "delete" => delete_task(tasks),
                _ => println!("no"),
            },
        _ => println!("Invalid command"),
    }

}


fn assign_task(tasks: &mut Vec<Task>, /* name: String, description: String, due: String */) -> () {
    let mut name: String = String::new();
    println!("Name: ");
    io::stdin().lock().read_line(&mut name).expect("Failed To read input");
    
    let mut description: String = String::new();
    println!("Description: ");
    io::stdin().lock().read_line(&mut description).expect("Failed To read input");

    let mut due: String = String::new();
    println!("Deadline: ");
    io::stdin().lock().read_line(&mut due).expect("Failed to read input");


    
    let new_task = Task {
        name: name,
        description: description,
        is_completed: false,
        time_created: Local::now(),
        date_due: due,
    };

    tasks.push(new_task);

}


fn delete_task(tasks: &mut Vec<Task>) -> () {
    let mut t: String = String::new();
    println!("Enter name of task to delete: ");
    io::stdin().lock().read_line(&mut t).expect("Failed to read input");
    let target = t.trim();

    if let Some(task) = tasks.iter_mut().position(|x| x.name == target) {
        tasks.remove(task);
    } else {
        println!("Task not found");
    }
}
