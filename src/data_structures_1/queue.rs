/// A linear DS following FIFO

pub struct Queue {
    front: isize,
    back: isize,
    items: Vec<isize>,
}

impl Queue {
    pub fn new(capacity: usize) -> Queue {
        Queue {
            front: -1,
            back: -1,
            items: vec![0; capacity],
        }
    }

    pub fn enqueue(&mut self, item_to_enqueue: isize) {
        if self.is_empty() {
            self.front = 0;
            self.back = self.back + 1;
            self.items[self.back as usize] = item_to_enqueue;
        } else {
            self.back = self.back + 1;
            self.items[self.back as usize] = item_to_enqueue;
        }
    }
    pub fn dequeue(&mut self) -> Option<isize> {
        if self.is_empty() {
            Some(-1)
        } else if self.back == self.front {
            self.items[self.front as usize] = 0;
            self.back = -1;
            self.front = -1;
            None
        } else {
            self.items[self.front as usize] = 0;
            self.front = self.front + 1;
            None
        }
    }

    fn is_empty(&self) -> bool {
        if self.back == -1 && self.front == -1 {
            true
        } else {
            false
        }
    }

    pub fn print_out(&self) {
        println!("{:?}", self.items)
    }
}
