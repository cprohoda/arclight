// /// Basic source code data structure for arclight
// /// Recursive data structure representing a given token and references to its super/previous and sub/next branches

use std::collections::LinkedList;

// fn main() {
//     let example_string = "Hi my name. is Chris\n\tusername";
//     let mut test_list: LinkedList<Element> = LinkedList::new();

//     build_list(&mut test_list, example_string);
//     println!("{}", test_list);
// }

// fn display(test_list: LinkedList<Element>, depth: &str) {
//     for node in test_list {
//         if node.list == Element::list {
//             display_list(node.list.unwrap(), &[".",depth].concat());
//         } else {
//             println!("{}{:?}", depth, node.token);
//         }
//     }
// }

// fn build(test_list: &mut LinkedList<Element>, tokens: Vec<String>) {
//     for token in tokens {
//         if token.ends_with('.') {
//             test_list.push_back(Element {
//                 list: build,
//                 token: String::from(token),
//             7});
//         } else {
//             test_list.push_back(Element {
//                 list: None,
//                 token: String::from(token),
//             });
//         }
//     }
// }

#[derive(PartialEq,Debug)]
struct Element<T> { // data structure is a linked list of these elements
    sub_list: Option<LinkedList<Element>>,
    token: T,
}

impl fmt::Display for Element {

}


