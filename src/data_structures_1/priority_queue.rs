//Some of the applications of a priority queue are:
/// Dijkstra's algorithm,
/// for implementing stack,
/// for load balancing and interrupt handling in an operating system,
/// for data compression in Huffman code,
pub struct PriorityQueue {}

impl PriorityQueue {
    pub fn heapify(arr: &mut Vec<isize>, size: isize, i: isize) {
        let mut largest = i;
        let left = (2 * i) + 1;
        let right = (2 * i) + 2;
        if left < size && arr[i as usize] < arr[left as usize] {
            largest = left
        }

        if right < size && arr[largest as usize] < arr[right as usize] {
            largest = right
        }

        if largest != i {
            let temp: isize = arr[i as usize];
            arr[i as usize] = arr[largest as usize];
            arr[largest as usize] = temp;
            PriorityQueue::heapify(arr, size, largest)
        }
    }

    pub fn insert() {}

    pub fn delete_node() {}

    pub fn print_out() {}
}
