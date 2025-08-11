// use std::env::current_exe;

pub struct Field {
    head: Link,
}

type Link = Option<Box<Node>>;
#[derive(Debug, PartialEq, Eq, Clone)]
struct Node {
    elem: Target,
    next: Link,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Target {
    pub size: u32,
    pub xp: u32,
}
impl Field {
    pub fn new() -> Self {
        Self { head: None }
    }
    pub fn push(&mut self, target: Target) {
        if self.head.is_none() {
            self.head = Some(Box::new(Node {
                elem: target,
                next: None,
            }))
        } else {
            let head = self.head.take();
            self.head = Some(Box::new(Node {
                elem: target,
                next: head,
            }))
        }
    }
    pub fn pop(&mut self) -> Option<Target> {
        let  current = self.head.take();
        if let Some(val) = current {
            let Target = val.elem;
            self.head = val.next;
            return Some(Target);
        }
        None
    }
    // pub fn pop(&mut self) -> Option<Target> {
    //     let mut current = &mut self.head;

    //     loop {
    //         match current {
    //             Some(node) if node.next.is_none() => {
    //                 let removed = current.take();
    //                 return Some(removed.unwrap().elem);
    //             }
    //             Some(node) => {
    //                 current = &mut node.next;
    //             }
    //             None => return None,
    //         }
    //     }
    // }
    pub fn peek(&self) -> Option<&Target> {
        match self.head.as_ref() {
            Some(val) => Some(&val.elem),
            None => None,
        }
    }
    pub fn peek_mut(&mut self) -> Option<&mut Target> {
        match self.head.as_mut() {
            Some(val) => Some(&mut val.elem),
            None => None,
        }
    }
}
