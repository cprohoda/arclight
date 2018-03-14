use std::fmt;
use std::collections::VecDeque;

use Photon::Photon;
use Parser::{parse,Tokens};
use Parser::{DEFINED,RETURN};

struct ArclightSyntaxTree {
    photons: Vec<Photon>,
    marker: Vec<usize>,
}

impl ArclightSyntaxTree {
    // fn row_tokens(&self) -> String {
    //     let mut row_tokens = String::new();
    //     let mut current_photon = &(self.marker);
        
    //     while Some(current_photon) {
    //         if row_tokens.len() > 0 {
    //             row_tokens.push(' ');
    //         }
    //         row_tokens.push(*current_photon.token.clone());
    //         current_photon = &(*current_photon.next_photon());
    //     }
    //     row_tokens
    // }

    pub fn new() -> ArclightSyntaxTree {
        ArclightSyntaxTree {
            photons: Vec::new(),
            marker: Vec::new(),
        }
    }

    pub fn build_at_marker(&mut self, tokens: Tokens) -> Result<(),AstBuilderError> {
        use Parser::TokenType;

        let current_photon = self.marker.pop();
        let marker_depth = 0i32;

        for token in tokens.iter() {
            match *token.token_type() {
                TokenType::Control => {
                    // change current photon
                },
                TokenType::Pass => {
                    // seperate photon
                },
                TokenType::Defined => {
                    self.photons[current_photon].token.push(DEFINED);
                    self.marker.push(current_photon);
                },
                TokenType::Return => {
                    self.photons[current_photon].token.push(RETURN);
                    self.marker.push(current_photon);
                },
                TokenType::Photon => {
                    // seperate photon
                },
            }
        }
        Ok(())
    }

    // fn to_alf(filename: &str) -> Result<> {

    // }

    // fn from_alf(filename: &str) -> Result<ArclightSyntaxTree,E> {

    // }
}

enum AstBuilderError {
    Unknown,
    UnmatchedDepth,
}


impl fmt::Debug for ArclightSyntaxTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Photons {:?}", self.photons)
    }
}

impl PartialEq for ArclightSyntaxTree {
    fn eq(&self, other: &ArclightSyntaxTree) -> bool {
        let mut equality: bool = self.photons.len() == other.photons.len();
        let mut iter_other = other.photons.iter();

        for self_token in &self.photons {
            if !equality { break; }
            equality = self_token == iter_other.next().unwrap();
        }

        equality
    }
}

// impl Iterator for ArclightSyntaxTree {
//     fn next(self) -> Result<Option<usize>, E> {
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

mod tests {
    use ArclightSyntaxTree::ArclightSyntaxTree;
    use Photon::Photon;
    use Parser::parse;

    fn defined_build_test() {
        let mut expected = ArclightSyntaxTree::new();
        expected.photons.push(Photon::new(Some("a.b".to_string())));

        let mut actual = ArclightSyntaxTree::new();
        actual.build_at_marker(parse("a.b").expect("Testing defined_build_test, actual parse"));

        assert_eq!(expected, actual);
    }
}
