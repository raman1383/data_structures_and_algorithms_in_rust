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

    pub fn insert(arr: &mut Vec<isize>, new_num: isize) {
        let mut size = arr.len();
        if size == 0 {
            arr.push(new_num);
            size = size + 1;
            print!("{}", size)
        } else {
            arr.push(new_num);
            size = size + 1;
            for mut i in 0..(size / 2) - 1 {
                PriorityQueue::heapify(arr, size as isize, i as isize);
                i = i - 1;
                print!("{}", i)
            }
        }
    }

    pub fn delete_node() {}

    pub fn print_out() {}
}
