/// Double-ended queue
pub struct Deque {
    items: Vec<isize>,
    rear: isize,
    front: isize,
    size: usize,
}

impl Deque {
    pub fn new(items: Vec<isize>) -> Deque {
        let size = items.len();
        Deque {
            items,
            rear: 0,
            front: -1,
            size,
        }
    }

    pub fn insert_at_front(&mut self, item: isize) {
        if self.items.is_empty() {
            if self.front == -1 {
                self.front = 0;
                self.rear = 0;
            } else if self.front == 0 {
                self.front = self.front - 1;
            } else {
                self.front = self.front - 1;
            }

            self.items[self.front as usize] = item;
        }
    }

    pub fn insert_at_back(&mut self, item: isize) {
        if self.is_empty() {
            if self.front == -1 {
                self.front = 0;
                self.rear = 0;
            } else if self.rear == (self.size - 1).try_into().unwrap() {
                self.rear = 0;
            } else {
                self.rear = self.rear + 1;
            }

            self.items[self.rear as usize] = item;
        }
    }

    pub fn remove_at_front(&mut self) {}
    pub fn remove_at_back(&mut self) {}

    pub fn is_empty(&self) -> bool {
        if self.items.len() == 0 {
            true
        } else {
            false
        }
    }
}
