/// A linear DS following LIFO
pub struct Stack {
    count: i128,
    top: i128,
    items: Vec<i128>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            items: Vec::with_capacity(10),
            count: 0,
            top: -1,
        }
    }

    pub fn push(&mut self, item_to_push: i128) {
        if !self.is_full() {
            self.top = self.top + 1;
            self.items[self.top as usize] = item_to_push;
            self.count = self.count + 1;
        } else {
            println!("!!! FULL !!!")
        }
    }

    pub fn pop(&mut self, item_to_pop: i128) {
        if !self.is_empty() {
            self.top = self.top - 1;
            self.items[self.top as usize] = item_to_pop;
            self.count = self.count - 1;
        } else {
            println!("!!! Empty !!!")
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

    // pub fn peek(&self) {}

    pub fn print_stack(&self) {
        for i in 0..self.count {
            println!("{}", self.items[i as usize])
        }
    }
}
