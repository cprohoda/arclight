use std::collections::LinkedList;

mod ArclightSyntaxTree;

fn main() {
    let example_string = "Hi my name. is Chris\n\tusername";
    let mut test_list: ArclightSyntaxTree = ArclightSyntaxTree::new();

    al_tree::new(&mut test_list, example_string);
    println!("{}", test_list);
}