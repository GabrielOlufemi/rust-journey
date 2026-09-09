struct Task {

    id: u32,
    title: String,
    desc: String,
    due_date: String,
    priority: Priority,
    status :  Status
}

enum Status {
    NotStarted,
    InProgress,
    Finished
}

enum Priority {
    Low,
    Medium,
    High
}

fn main() { 

}

