pub mod blockchain;
pub mod data_structures_1;
pub mod data_structures_2;
pub mod dynamic_programming;
pub mod graph_based_dsa_1;
pub mod greedy_algs;
pub mod lib_peer_to_peer;
pub mod other_algs;
pub mod sorting_and_searching_algs;
pub mod tree_based_dsa_1;
pub mod tree_based_dsa_2;
fn main() {
    let mut ll = data_structures_2::linked_list::LinkedList {
        value: 1,
        next: data_structures_2::linked_list::Address::Nil,
    };

    ll.append(2);
    ll.append(3);
    // ll.list();

    for i in (0..11).step_by(2) {
        println!("{i} It is me ")
    }
}
