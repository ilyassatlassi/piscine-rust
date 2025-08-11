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
        let mut current = self.node.take();
        let mut prev = None;
        loop {
            match current {
                // Some(val) if val.next_person.is_none() => {

                //     // return Some((removed.name, removed.discount));
                // }
                Some(mut val) => {
                    let next = val.next_person.take();
                    val.next_person = prev;
                    prev = Some(val);
                    current = next;
                    // current = &mut val.next_person;
                }
                None => break,
            }
        }
        self.node = prev;
    }
    pub fn rm(&mut self) -> Option<(String, i32)> {
        let mut current = &mut self.node;
        loop {
            match current {
                Some(val) if val.next_person.is_none() => {
                    let removed = current.take().unwrap();
                    return Some((removed.name, removed.discount));
                }
                Some(val) => {
                    current = &mut val.next_person;
                }
                None => return None,
            }
        }
        // None
    }
    pub fn search(&self, name: &str) -> Option<(String, i32)> {
        let mut current = &self.node;
        loop {
            match current {
                Some(val) if name == val.name => {
                    // let removed = current.take().unwrap();
                    return Some((val.name.to_string(), val.discount));
                }
                Some(val) => {
                    current = &val.next_person;
                }
                None => return None,
            }
        }
    }
}
