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
        if self.items.is_empty() {}
    }
    pub fn insert_at_back(&mut self, item: isize) {
        self.items.push(item);
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
