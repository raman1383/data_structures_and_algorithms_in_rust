pub mod data_structures_1;
pub mod data_structures_2;
pub mod dynamic_programming;
pub mod graph_based_dsa_1;
pub mod greedy_algs;
pub mod other_algs;
pub mod sorting_and_searching_algs;
pub mod tree_based_dsa_1;
pub mod tree_based_dsa_2;

fn main() {
    // swap without temp

    let mut x = 10;
    let mut y = 5;

    x = x + y;
    y = x - y;
    x = x - y;

    println!("{x} {y}");
}
