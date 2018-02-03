use std::fmt;

mod Elements;

struct ArclightSyntaxTree {
    elements: Option<Vec<Element>>,
    marker_depth: usize,
    marker: Option<&Eleme>,
}

impl ArclightSyntaxTree {
    // fn row_tokens(&self) -> String {
    //     let mut row_tokens = String::new();
    //     let mut current_element = &(self.marker);
        
    //     while Some(current_element) {
    //         if row_tokens.len() > 0 {
    //             row_tokens.push(' ');
    //         }
    //         row_tokens.push(*current_element.token.clone());
    //         current_element = &(*current_element.next_element());
    //     }
    //     row_tokens
    // }

    fn new() -> ArclightSyntaxTree {
        ArclightSyntaxTree {
            elements: None,
            marker_depth: 0,
            marker: None,
        }
    }

    fn build(&self, from: String) -> Self {

    }

    // fn to_alf(filename: &str) -> Result<> {

    // }

    // fn from_alf(filename: &str) -> Result<ArclightSyntaxTree,E> {

    // }
}

// impl fmt::Display for ArclightSyntaxTree {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         while self.has_next() {
//             if 

//         }
//     }
// }

impl Iterator for ArclightSyntaxTree {
    fn next(self) -> Result<Option<usize>, E> {
        // down if Some(marker.down)
        //     self.marker_depth += 1;
        //     self.marker = self.marker.down
        //     Result<Success>
        // else right if Some(marker.right)
        //     self.marker = self.marker.right
        //     Result<Success>
        // else right if Some(marker.up.right)
        //     marker_depth -= 1
        //     self.marker = self.marker.up.right
        //     Result<Success>
        // else
        //     Result<Failure>
    }

    // fn has_next(&self) -> Result<()> {}
}