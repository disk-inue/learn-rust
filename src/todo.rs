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
}

pub fn exec() {
    let todo_list: Vec<Todo> = Vec::new();

    println!("start todo");
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
            let todo_list = add(&todo_list, input("title > "));
            println!("{:?}", todo_list);
        }
        2 => {}
        3 => {}
        4 => {}
        _ => {
            println!("end todo");
            return;
        }
    }
}

fn add(todo_list: &Vec<Todo>, title: String) -> Vec<Todo> {
    let new_todo = Todo::new(title);
    let mut tmp_list = todo_list.to_vec();
    tmp_list.push(new_todo);
    tmp_list
}

fn edit() {}

fn done() {}
fn delete() {}
