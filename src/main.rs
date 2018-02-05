#![feature(generators)]


mod Parser;

fn main() {
    let example_string = "Hi my name. \"hahahaha\" is Chris\n\tusername";

    let blah = Parser::parse(example_string);

    println!("{:?}", blah)
    // let mut test_list: ArclightSyntaxTree = ArclightSyntaxTree::new();

    // al_tree::new(&mut test_list, example_string);
    // println!("{}", test_list);
}