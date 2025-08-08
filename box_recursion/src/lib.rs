#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Role {
    CEO,
    Manager,
    Worker,
}

impl From<&str> for Role {
    fn from(value: &str) -> Self {
        if value == "CEO" {
            return self::Role::CEO;
        }

        if value == "Manager" {
            return self::Role::Manager;
        }

        if value == "Normal Worker" {
            return self::Role::Worker;
        }
        todo!()
    }
}

#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug, Clone, PartialEq)]
pub struct Worker {
    pub role: Role,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> Self {
        Self { grade: None }
    }

    pub fn add_worker(&mut self, name: &str, role: &str) {
        // match self.grade {
        //     None => {
        //         self.grade = Some(Box::new(Worker {
        //             name: name.to_string(),
        //             role: Role::from(role),
        //             next: None,
        //         }))
        //     }
        //     _ => {
        // let mut last = &mut self.grade;
        // let mut new = Some(Box::new(Worker {
        //     name: name.to_string(),
        //     role: Role::from(role),
        //     next: None,
        // }));
        // if let Some(val) =last  {
        // new.unwrap().next = val;
        // }

        // while let Some(val) = last {
        //     match val.next {
        //         None => {
        //             val.next = Some(Box::new(Worker {
        //                 name: name.to_string(),
        //                 role: Role::from(role),
        //                 next: None,
        //             }));
        //             break;
        //         }
        //         _ => {
        //             last = &mut val.next;
        //         }
        //     }
        // }
        // }

        // }
        let new_worker = Box::new(Worker {
            name: name.to_string(),
            role: Role::from(role),
            next: self.grade.take(),
        });
        self.grade = Some(new_worker);
    }
    pub fn remove_worker(&mut self) -> Option<String> {
        let  link =  self.grade.take();
        if link.is_none() {
            return None;
        }
        if let Some(worker) = link {
            let name = worker.name;
            self.grade = worker.next;
            return Some(name);
        }

     
        todo!()
    }

    pub fn last_worker(&self) -> Option<(String, Role)> {
        let  index = &self.grade;
        if let Some(val) = index  {
                    return Some((val.name.clone(), val.role));
        }
        // while let Some(val) = index {
        //     match val.next {
        //         None => {
        //             // let res = index;

        //             return Some((val.name.clone(), val.role));
        //         }
        //         _ => {
        //             index = &val.next;
        //         }
        //     }
        // }
        todo!()
    }
}
// use box_recursion::*;

#[test]
fn test_new() {
    let list = WorkEnvironment::new();
    assert!(list.grade.is_none());
}

#[test]
fn test_one_worker() {
    let mut list = WorkEnvironment::new();
    list.add_worker("Marie", "CEO");
    list.remove_worker();
    assert!(list.grade.is_none());
}

#[test]
fn test_two_workers() {
    let mut list = WorkEnvironment::new();
    list.add_worker("Marie", "CEO");
    list.add_worker("Monica", "Manager");
    list.remove_worker();

    assert_eq!(list.grade.as_ref().unwrap().role, Role::CEO);
    assert_eq!(list.grade.as_ref().unwrap().name, "Marie".to_owned());
}

#[test]
fn test_more_workers() {
    let mut list = WorkEnvironment::new();
    list.add_worker("Marie", "CEO");
    list.add_worker("Monica", "Manager");
    list.add_worker("Ana", "Normal Worker");
    list.add_worker("Alice", "Normal Worker");
    list.remove_worker();

    assert_eq!(list.grade.as_ref().unwrap().role, Role::Worker);
    assert_eq!(list.grade.as_ref().unwrap().name, "Ana".to_owned());

    list.remove_worker();
    list.remove_worker();

    assert_eq!(list.grade.as_ref().unwrap().role, Role::CEO);
    assert_eq!(list.grade.as_ref().unwrap().name, "Marie".to_owned());
}

#[test]
fn test_last_worker() {
    let mut list = WorkEnvironment::new();
    list.add_worker("Marie", "CEO");
    list.add_worker("Monica", "Manager");
    list.add_worker("Ana", "Normal Worker");
    list.add_worker("Alice", "Normal Worker");

    assert_eq!(list.last_worker(), Some(("Alice".to_owned(), Role::Worker)));

    list.remove_worker();
    assert_eq!(list.last_worker(), Some(("Ana".to_owned(), Role::Worker)));

    list.remove_worker();
    assert_eq!(
        list.last_worker(),
        Some(("Monica".to_owned(), Role::Manager))
    );

    list.remove_worker();
    assert_eq!(list.last_worker(), Some(("Marie".to_owned(), Role::CEO)));
}
