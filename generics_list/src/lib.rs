#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None }
    }

    pub fn push(&mut self, value: T) {
        if self.head.is_none() {
            self.head = Some(Node { value, next: None });
        } else {
            let current = self.head.take().unwrap();

            self.head = Some(Node {
                value,
                next: Some(Box::new(current)),
            });
        }
    }

    pub fn pop(&mut self) {
        let current = self.head.take().unwrap().next.unwrap();
        self.head = Some(*current);
    }

    pub fn len(&self) -> usize {
        let mut res = 0;
        if !self.head.is_none() {
            res = 1;
        }

        let mut current = &self.head.as_ref().unwrap().next;
        while let Some(val) = current {
            res += 1;
            current = &val.next;
        }
        res
    }
}
