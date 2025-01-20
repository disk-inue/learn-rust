use std::collections::HashMap;

use crate::common::input;

#[derive(Clone, Debug)]
enum Status {
    Todo,
    Done,
}

#[derive(Clone, Debug)]
struct Todo {
    id: i32,
    title: String,
    done: Status,
}

impl Todo {
    fn new(title: String) -> Todo {
        Todo {
            id: 0,
            title,
            done: Status::Todo,
        }
    }
    fn done(&self) -> Todo {
        Todo {
            id: self.id,
            title: self.title.clone(),
            done: Status::Done,
        }
    }
}

pub fn exec() {
    let mut todo_map: HashMap<u32, Todo> = HashMap::new();

    println!("start todo");
    loop {
        println!("select menu");
        println!("1. add\n2. edit\n3. done\n4. delete");

        let select_number: u8 = match input(" > ").parse() {
            Ok(num) => num,
            Err(_) => {
                println!("end todo");
                return;
            }
        };

        match select_number {
            1 => {
                let todo_map = add(&mut todo_map, input("title > "));
                println!("{:?}", todo_map);
            }
            2 => {}
            3 => {
                for (key, value) in &todo_map {
                    println!("{}: {}", key, value.title);
                }
                let target_number: u32 = match input("number > ").parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("end todo");
                        return;
                    }
                };

                let todo_map = done(&mut todo_map, target_number);
                println!("{:?}", todo_map);
            }
            4 => {}
            _ => {
                println!("end todo");
                return;
            }
        }
    }
}

fn add(todo_map: &mut HashMap<u32, Todo>, title: String) -> HashMap<u32, Todo> {
    let new_todo = Todo::new(title);
    let id = match u32::try_from(todo_map.len()) {
        Ok(num) => num,
        Err(_) => return todo_map.clone(),
    };
    &todo_map.insert(id, new_todo);
    todo_map.clone()
}

fn edit() {}

fn done(todo_map: &mut HashMap<u32, Todo>, number: u32) -> HashMap<u32, Todo> {
    let binding = todo_map.get(&number);
    let target_todo = match &binding {
        Some(todo) => todo,
        None => return todo_map.clone(),
    };
    let done_todo = &target_todo.done();
    &todo_map.entry(number).or_insert(done_todo.clone());
    todo_map.clone()
}

fn delete() {}
