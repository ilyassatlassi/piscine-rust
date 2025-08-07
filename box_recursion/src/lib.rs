#[derive(Debug, PartialEq)]
pub enum Role {
    CEO,
    Manager,
    Worker,
}

// impl From<&str> for Role {}

#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> Self {
        Self { grade: None }
    }

    pub fn add_worker(&mut self, name: &str, role: &str) {
        match self.grade {
            None => {
                self.grade = Some(Box::new(Worker {
                    name: name.to_string(),
                    role: role.to_string(),
                    next: None,
                }))
            }
            _ => {
                let mut last = &mut self.grade;
                while let Some(val) = last {
                    match val.next {
                        None => {
                            val.next = Some(Box::new(Worker {
                                name: name.to_string(),
                                role: role.to_string(),
                                next: None,
                            }));
                            break;
                        }
                        _ => {
                            last = &mut val.next;
                        }
                    }
                }
            }
        }
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        let mut index = &mut self.grade;
        while let Some(val) = index {
            match val.next {
                None => {
                    // let res = index;
                    
                    return Some(val.name.clone());
                }
                _ => {
                    index = &mut val.next;
                }
            }
        }

        // index
        todo!()
    }

    pub fn last_worker(&self) -> Option<(String, Role)> {
        todo!()
    }
}
