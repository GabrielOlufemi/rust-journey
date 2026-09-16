use std::{clone, collections::HashMap, hash::Hash};
use std::env::args;

#[derive(Debug)]
struct ToDoApp {
    tasks: HashMap<u32, Task>,
    order: Vec<u32>,
    next_id: u32

}

impl ToDoApp {
    fn add_task (&mut self, title: String, desc: String, due_date: String, priority: Priority, status: Status) {
        
        // task retrieval p
        let task_id = self.next_id;
        self.next_id += 1;

        // task object generation
        let new_task = Task {
            id: task_id,
            title: title,
            desc: desc,
            due_date: due_date,
            priority: priority,
            status: status
        };

        // creating mapping
        self.tasks.entry(task_id).or_insert(new_task);

        // pushing id to order vector
        self.order.push(task_id)


    }

    fn remove_task (&mut self, id: u32) {

        // removal from hashmap
        self.tasks.remove(&id);

        // removal from vectora
        self.order.retain(|x| *x != id);
    }

    fn list_tasks(&self) -> Vec<&Task> {

        let mut task_list = vec![];

        for item in &self.order {
            if let Some(task) = self.tasks.get(item) {
                task_list.push(task);
            }
        }

        task_list
    }

    fn complete_task(&mut self, id: u32) {

        // modify hashmap i guess
        let task = self.tasks.get_mut(&id);


        if let Some(task) = task {
            task.status= Status::Finished;
        }        

    }
}

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
struct Task {
    id: u32,
    title: String,
    desc: String,
    due_date: String,
    priority: Priority,
    status :  Status
}

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
enum Status {
    NotStarted,
    InProgress,
    Finished
}

#[derive(Debug,Clone,Copy,PartialEq, Eq,Hash)]
enum Priority {
    Low,
    Medium,
    High
}


fn main() { 

    let mut app = ToDoApp{
        tasks: HashMap::new(),
        order: Vec::new(),
        next_id: 1,
    };





    // reading task type
    let command = args().skip(1).next();


    // read task argument
    let cmd_arg = args().skip(2).next();


    match command {
        Some(cmd) => {
            match cmd.as_str() {
                "add" => {
                            println!("add called");

                            match cmd_arg {

                                Some(title) => {
                                    app.add_task(title.clone(), String::new(), String::new(), Priority::Low, Status::NotStarted);
                                   
                                    println!("title received: {}", title)
                                
                                },
                                None => println!("no argument provided")

                            }
                        },
                "remove" => {
                            println!("remove called");

                            match cmd_arg {
                                Some(id_str) => {
                                    match id_str.parse::<u32>() {
                                        Ok(id) => app.remove_task(id),
                                        Err(_) => println!("invalid id: nan")
                                    }
                                },
                                None => println!("no argument provided")
                            }
                        },
                "list" => {
                            println!("list called");

                            let task_list = app.list_tasks();

                            for item in task_list {
                                println!("{:?}", item);
                            }
                        }
                "complete" => {
                                println!("complete called");
                                
                                match cmd_arg {
                                    Some(id_str) => {
                                        match id_str.parse::<u32>() {
                                            Ok(id) => {
                                                app.complete_task(id);
                                            },
                                            Err(_) => println!("invalid id: nan")
                                        }
                                    },
                                    None => println!("no arugment provided")
                                }
                            },

                _ => println!("unrecognized command: {cmd}")
            }
        },
        None => println!("no command provided")
    }




}

// group by id

fn group_by_id(tasks: &Vec<Task>) -> HashMap<u32, Vec<Task>> {

    let mut map = HashMap::new();

    for task in tasks {
        let category = map.entry(task.id).or_insert(vec![]);
        category.push(task.clone());
    }

    map

}

fn find_by_id(tasks: &HashMap<u32, Task>, id:u32) -> Option<&Task> {

    tasks.get(&id)

}

fn id_in_vector<'a>(tasks: &'a Vec<Task>, id:&u32) -> Option<&'a Task> { 

    for task in tasks { 
        if task.id == *id { 
            return Some(task)
        }
    }

    None
}

fn count_tasks(tasks: &Vec<Task>) -> HashMap<Status, u32> {


    let mut map = HashMap::new();
    
    for task in tasks {
        let each_status = map.entry(task.status).or_insert(0);
        *each_status += 1;
    }

    return map;

}

fn group_tasks(tasks: &Vec<Task>) -> HashMap<Status, Vec<Task>>{

    let mut map = HashMap::new();

    for task in tasks { 

            let grouped = map.entry(task.status).or_insert(vec![]);

            grouped.push(task.clone());
    }

    map

}
