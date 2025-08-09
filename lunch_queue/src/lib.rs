#[derive(Debug)]
pub struct Queue {
    pub node: Link,
}

pub type Link = Option<Box<Person>>;

#[derive(Debug)]
pub struct Person {
    pub discount: i32,
    pub name: String,
    pub next_person: Link,
}

impl Queue {
    pub fn new() -> Queue {
        Queue { node: None }
    }
    pub fn add(&mut self, name: String, discount: i32) {
        if self.node.is_none() {
            self.node = Some(Box::new(Person {
                discount,
                name,
                next_person: None,
            }))
        } else {
            let prev = self.node.take();

            self.node = Some(Box::new(Person {
                discount,
                name,
                next_person: prev,
            }))
        }
    }
    pub fn invert_queue(&mut self) {
        
    }
    pub fn rm(&mut self) -> Option<(String, i32)> {
        let mut current = &mut self.node;
        while let Some(val) = current {
            if val.next_person.is_none() {
                let removed = current.take().unwrap();
                return Some((removed.name, removed.discount));
            }
            current = &mut current.as_mut().unwrap().next_person;
        }

        None
    }
    pub fn search(&self, name: &str) -> Option<(String, i32)> {
        let mut current = &self.node;
        while let Some(val) = current {
            if val.name == name {
                return Some((val.name.to_string(), val.discount));
            }
            current = &current.as_ref().unwrap().next_person;
        }
    None
    }
}
