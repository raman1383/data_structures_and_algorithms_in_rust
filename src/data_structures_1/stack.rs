/// A linear DS following LIFO
pub struct Stack {
    max_size: usize,
    items: Vec<isize>,
}

impl Stack {
    pub fn with_capacity(max_size: usize) -> Self {
        Self {
            max_size,
            items: Vec::with_capacity(max_size),
        }
    }

    pub fn pop(&mut self) -> Option<isize> {
        self.items.pop()
    }

    pub fn push(&mut self, item: isize) -> bool {
        if self.items.len() == self.max_size {
            return false;
        }
        self.items.push(item);
        return true;
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn peek(&self) -> Option<&isize> {
        self.items.last()
    }

    pub fn print_out(&self) {
        println!("{:#?}", self.items)
    }
}
