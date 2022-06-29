pub struct CircularQueue {
    items: Vec<isize>,
    pub rear_ptr: isize,
    pub front_ptr: isize,
    capacity: usize,
}

impl CircularQueue {
    pub fn new(capacity: usize) -> CircularQueue {
        CircularQueue {
            items: vec![0; capacity],
            rear_ptr: -1,
            front_ptr: -1,
            capacity,
        }
    }

    pub fn dequeue(&mut self) -> Option<isize> {
        if self.is_empty() {
            Some(self.front_ptr)
        } else if self.rear_ptr == self.front_ptr {
            self.front_ptr = -1;
            self.rear_ptr = -1;
            None
        } else {
            if self.front_ptr == (self.capacity - 1).try_into().unwrap() {
                self.front_ptr = (self.front_ptr + 1) % self.capacity as isize;
                self.items[self.front_ptr as usize] = 0;
            } else {
                self.items[self.front_ptr as usize] = 0;
                self.front_ptr = self.front_ptr + 1;
            }
            None
        }
    }

    pub fn enqueue(&mut self, item_to_enqueue: isize) {
        if self.is_empty() {
            self.front_ptr = 0;
            self.rear_ptr = self.rear_ptr + 1;
            self.items[self.rear_ptr as usize] = item_to_enqueue;
        } else {
            if self.rear_ptr == (self.capacity - 1).try_into().unwrap() {
                self.rear_ptr = (self.rear_ptr + 1) % self.capacity as isize
            } else {
                self.rear_ptr = self.rear_ptr + 1;
            }
            self.items[self.rear_ptr as usize] = item_to_enqueue;
        }
    }

    pub fn is_empty(&self) -> bool {
        if self.front_ptr == -1 && self.rear_ptr == -1 {
            true
        } else {
            false
        }
    }

    pub fn print_out(&self) {
        println!("{:?}", self.items)
    }
}
