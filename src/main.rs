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

    println!("x = {x}, y = {y}");

    println!("{}", other_algs::fun::find_factorial_2(5));
    println!("{}", other_algs::fun::sum_of_first_10_pow_11_naturals());

    let mut stack_1 = data_structures_1::stack::Stack::new();
    stack_1.push(44);
    stack_1.push(32);
    stack_1.push(4);
    stack_1.push(54);
    stack_1.print_stack();
    stack_1.pop();
    stack_1.pop();
    stack_1.pop();
}
