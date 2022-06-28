/// A linear DS following LIFO
pub struct Stack {
    count: usize,
    top: isize,
    items: Vec<isize>,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            count: 0,
            top: -1,
            items: Vec::with_capacity(10),
        }
    }

    pub fn push(&mut self, item_to_push: isize) {
        if self.is_full() {
            println!("!!! FULL !!!")
        } else {
            self.top = self.top + 1;
            self.count = self.count + 1;
            self.items.push(item_to_push);
        }
    }

    pub fn pop(&mut self) {
        if self.is_empty() {
            println!("!!! Empty !!!")
        } else {
            self.top = self.top - 1;
            self.count = self.count - 1;
            self.items.pop();
        }
    }

    pub fn is_empty(&self) -> bool {
        if self.top == -1 {
            true
        } else {
            false
        }
    }

    pub fn is_full(&self) -> bool {
        if self.top == 9 {
            true
        } else {
            false
        }
    }

    // pub fn peek(&self) {}

    pub fn print_stack(&self) {
        for i in 0..self.top + 1 {
            print!("[ {:?} ]", self.items[i as usize])
        }
    }
}
