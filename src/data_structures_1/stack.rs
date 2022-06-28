/// A linear DS following LIFO
pub struct Stack {
    top_ptr: Option<i128>,
    count: i128,
    // elements
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            top_ptr: None,
            count: -1,
        }
    }

    pub fn push(&mut self) {
        if !self.is_full() {
            self.count = self.count + 1;
            self.top_ptr = Some(self.top_ptr.unwrap() + 1)
        }
    }

    pub fn pop(&mut self) {
        if !self.is_empty() {
            self.count = self.count - 1;
            self.top_ptr = Some(self.top_ptr.unwrap() - 1)
        }
    }

    pub fn is_empty(&self) -> bool {
        if self.count == -1 {
            true
        } else {
            false
        }
    }

    pub fn is_full(&self) -> bool {
        if self.count == -1 {
            false
        } else {
            true
        }
    }

    pub fn peek(&self) {}
}
