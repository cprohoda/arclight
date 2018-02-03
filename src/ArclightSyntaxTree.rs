extern crate core;

use std::fmt;
use core::ptr::Shared;

#[derive(PartialEq,Debug)]
struct Element { // data structure is a linked list of these elements
    sub_list: Option<Shared<Element<String>>>,
    next_list: Option<Shared<Element<String>>>,
    prev_list: Option<Shared<Element<String>>>,
    super_list: Option<Shared<Element<String>>>,
    token: String,
}

impl Element {
    fn new(token: String) -> Self {
        Element{
            sub_list: None,
            next_list: None,
            prev_list: None,
            super_list: None,
            token: token,
        }
    }
}

struct ArclightSyntaxTree {
    head: Option<Shared<Element<String>>>,
    len: usize,
    marker_depth: u32,
    marker: Option<Shared<Element<String>>>,
}

impl ArclightSyntaxTree {
    fn row_tokens(&self) -> String {
        let mut row_tokens = String::new();
        let mut current_element = &(self.marker);
        
        while Some(current_element) {
            if row_tokens.len() > 0 {
                row_tokens.push(' ');
            }
            row_tokens.push(*current_element.token.clone());
            current_element = &(*current_element.next_element());
        }
        row_tokens
    }

    fn new() -> Self {
        ArclightSyntaxTree {
            head: None,
            len: 0,
            marker_depth: 0u32,
            marker: None,
        }
    }

    // fn build() -> Self {

    // }
}

// impl fmt::Display for ArclightSyntaxTree {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         while self.has_next() {
//             if 

//         }
//     }
// }

// impl Iterator for ArclightSyntaxTree {
//     fn next(&mut self) -> Result<()> {
//         // down if Some(marker.down)
//         //     self.marker_depth += 1;
//         //     self.marker = self.marker.down
//         //     Result<Success>
//         // else right if Some(marker.right)
//         //     self.marker = self.marker.right
//         //     Result<Success>
//         // else right if Some(marker.up.right)
//         //     marker_depth -= 1
//         //     self.marker = self.marker.up.right
//         //     Result<Success>
//         // else
//         //     Result<Failure>
//     }

//     // fn has_next(&self) -> Result<()> {}
// }