use std::{clone, collections::HashMap, hash::Hash};

use crate::Status::Finished;

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


    let v = vec![
        Task {
            id: 1,
            title: String::from("Task 1"),
            desc: String::from("Description for Task 1"),
            due_date: String::from("2023-12-31"),
            priority: Priority::High,
            status: Status::NotStarted
        },
        Task {
            id: 2,
            title: String::from("Task 2"),
            desc: String::from("Description for Task 2"),
            due_date: String::from("2023-11-30"),
            priority: Priority::Medium,
            status: Status::InProgress
        },
        Task {
            id: 3,
            title: String::from("Task 3"),
            desc: String::from("Description for Task 3"),
            due_date: String::from("2023-10-15"),
            priority: Priority::Low,
            status: Status::Finished
        },
        Task {
            id: 4,
            title: String::from("Task 4"),
            desc: String::from("Description for Task 4"),
            due_date: String::from("2023-10-15"),
            priority: Priority::Low,
            status: Status::NotStarted
        }
    ];


    let result = count_tasks(&v);


    println!("{result:#?}");

    let grouped_tasks = group_tasks(&v);

    println!("Grouped tasks p");
    println!("{grouped_tasks:#?}");

    let grouped_id_tasks = group_by_id(&v);

    println!("Grouped tasks by id");
    println!("{grouped_id_tasks:#?}");


    // search for id in vector
    let search_id = 2;

    let found = id_in_vector(&v, &search_id);
    println!("{found:#?}");

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
