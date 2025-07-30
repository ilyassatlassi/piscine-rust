mod err;
pub use err::{ParseErr, ReadErr};
use json::parse;
use std::{ error::Error};

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
        let file = std::fs::read_to_string(path);
        match file {
            Ok(content) => match parse(&content) {
                Ok(parsed) => {
                    let mut tasks = Vec::new();
                    if parsed["tasks"].is_empty() {
                        return Err(Box::new(err::ParseErr::Empty));
                    }
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
                Err(e) => Err(Box::new(err::ParseErr::Malformed(Box::new(e)))),
            },
            Err(e) => Err(Box::new(err::ReadErr {
                child_err: Box::new(e),
            })),
        }

        // todo!()
    }
}

use json::{object, JsonValue};
use std::io::{self, Write};
use tempfile::NamedTempFile;

fn write_and_read_test(
    write_f: impl FnOnce(&mut NamedTempFile),
) -> Result<TodoList, Box<dyn Error>> {
    let mut file = NamedTempFile::new().unwrap();
    write_f(&mut file);

    let path = file.path().to_str().unwrap();
    TodoList::get_todo(path)
}

#[inline]
fn json_to_file_to_mem(obj: JsonValue) -> Result<TodoList, Box<dyn Error>> {
    write_and_read_test(|f| obj.write(f).unwrap())
}

#[test]
fn test_valid_todo() {
    let r#struct = TodoList {
        title: "todo list for something".to_owned(),
        tasks: vec![
            Task {
                id: 0,
                description: "do this".to_owned(),
                level: 0,
            },
            Task {
                id: 1,
                description: "do that".to_owned(),
                level: 5,
            },
        ],
    };

    let obj = object! {
        "title" : "todo list for something",
        "tasks": [
            { "id": 0, "description": "do this", "level": 0 },
            { "id": 1, "description": "do that", "level": 5 }
        ]
    };

    assert_eq!(r#struct, json_to_file_to_mem(obj).unwrap());
}

#[test]
fn test_empty_tasks() {
    let obj = object! {
        "title" : "empty tasks",
        "tasks": []
    };

    let result = json_to_file_to_mem(obj).unwrap_err();

    assert!(matches!(result.downcast_ref().unwrap(), ParseErr::Empty));
    assert!(result.source().is_none());
    assert_eq!(result.to_string(), "Failed to parse todo file");
}

#[test]
fn test_read_err() {
    let result = TodoList::get_todo("invalid_file.json").unwrap_err();

    let ReadErr { child_err } = result.downcast_ref().unwrap();

    assert!(child_err.is::<io::Error>());
    assert!(result.source().unwrap().is::<io::Error>());
    assert_eq!(result.to_string(), "Failed to read todo file");
}

#[test]
fn test_parse_err_malformed() {
    let result = write_and_read_test(|f| f.write_all(r#"{"something": ,}"#.as_bytes()).unwrap())
        .unwrap_err();

    let ParseErr::Malformed(e) = result.downcast_ref::<ParseErr>().unwrap() else {
        panic!()
    };

    assert!(e.is::<json::Error>());
    assert!(result.source().unwrap().is::<ParseErr>());
    assert_eq!(result.to_string(), "Failed to parse todo file");
}