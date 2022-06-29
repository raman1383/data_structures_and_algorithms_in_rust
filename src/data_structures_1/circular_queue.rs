pub struct CircularQueue {
    items: Vec<isize>,
    rear_ptr: isize,
    front_ptr: isize,
}

impl CircularQueue {
    pub fn new(capacity: usize) -> CircularQueue {
        CircularQueue {
            items: vec![0; capacity],
            rear_ptr: -1,
            front_ptr: -1,
        }
    }

    pub fn dequeue(&mut self) {}

    pub fn enqueue(&mut self, item_to_enqueue: isize) {}

    pub fn print_out(&self) {}
}
