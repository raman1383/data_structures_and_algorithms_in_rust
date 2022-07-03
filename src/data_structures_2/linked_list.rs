#![allow(non_camel_case_types)]

#[derive(Clone)]
pub struct LinkedList {
    value: u32,
    next: Address,
}

impl LinkedList {
    pub fn append(&mut self, elem: u32) {
        match self.next {
            Address::address(ref mut next_address) => {
                next_address.append(elem);
            }
            Address::Nil => {
                let node = LinkedList {
                    value: elem,
                    next: Address::Nil,
                };
                self.next = Address::address(Box::new(node))
            }
        }
    }

    pub fn delete(&mut self, elem: u32) {
        match self.next {
            Address::address(ref mut next_address) => {
                if next_address.value == elem {
                    println!("Deleting value {}", next_address.value);
                    self.next = next_address.next.clone();
                } else {
                    next_address.delete(elem);
                }
            }
            Address::Nil => {
                if self.value == elem {
                    self.value = 0;
                } else {
                    println!("Element {} does not exist in the linked list", elem);
                }
            }
        }
    }
}

#[derive(Clone)]
enum Address {
    address(Box<LinkedList>),
    Nil,
}
