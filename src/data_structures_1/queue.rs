use std::borrow::BorrowMut;

/// A linear DS following FIFO
struct QueueNode<T> {
    value: T,
    next: Option<Box<QueueNode<T>>>,
}

impl<T> QueueNode<T> {
    fn new(value: T) -> Self {
        QueueNode { value, next: None }
    }
}

pub struct Queue<T> {
    end: Option<QueueNode<T>>,
}

impl<T> Queue<T> {
    pub fn remove(&mut self) -> Option<T> {
        if !self.is_empty() {
            let end = std::mem::take(&mut self.end).unwrap();
            if let Some(next) = end.next {
                self.end = Some(*next);
            }
            Some(end.value)
        } else {
            None
        }
    }

    pub fn add(&mut self, value: T) {
        let new_node = QueueNode::new(value);
        if let Some(end) = &mut self.end {
            let mut start = end;
            loop {
                if let Some(_) = &start.next {
                    start = (start.next.as_mut().unwrap()).borrow_mut();
                } else {
                    break;
                }
            }
            start.next = Some(Box::new(new_node));
        } else {
            self.end = Some(new_node);
        }
    }

    pub fn is_empty(&self) -> bool {
        match self.end {
            None => true,
            _ => false,
        }
    }
}
