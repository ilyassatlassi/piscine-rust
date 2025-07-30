// mod err;
use json::parse;
use std::error::Error;

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        match parse(path) {
            Ok(parsed) => {
                let mut tasks = Vec::new();

                for task in parsed["tasks"].members() {
                    let task = Task {
                        id: task["id"].as_u32().unwrap(),
                        description: task["description"].to_string(),
                        level: task["level"].as_u32().unwrap(),
                    };

                    tasks.push(task);
                }
                let new_todo = TodoList {
                    title: parsed["title"].to_string(),
                    tasks: tasks,
                };
                Ok(new_todo)
            }
            Err(e) => Err(Box::new(e)),
        }
        // todo!()
    }
}
