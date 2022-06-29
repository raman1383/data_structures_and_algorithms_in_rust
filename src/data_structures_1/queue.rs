/// A linear DS following FIFO
pub struct Queue {
    rear_ptr: isize,
    front_ptr: isize,
    items: Vec<isize>,
}

impl Queue {
    pub fn new() -> Self {
        Queue {
            rear_ptr: -1,
            front_ptr: -1,
            items: Vec::with_capacity(10),
        }
    }

    pub fn enqueue(&mut self) {}

    pub fn dequeue() {}

    pub fn peek() {}

    pub fn is_empty(&self) -> bool {
        if self.front_ptr == -1 && self.rear_ptr == -1 {
            true
        } else {
            false
        }
    }
}
