use std::fmt;
use std::collections::VecDeque;

use Photon::Photon;
use Parser::{parse,Tokens};

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

    pub fn build_at_marker(&self, tokens: Tokens) -> Result<Self,AstBuilderError> {
        use Parser::TokenType;

        let mut current_photon = self.marker.pop();
        let mut marker_depth = 0i32;

        for token in tokens {
            match token.token_type {
                TokenType::Control => {
                    // change current photon
                },
                TokenType::Pass => {
                    // seperate photon
                },
                TokenType::Defined => {
                    // append this token and next token to current photon's token
                },
                TokenType::Return => {
                    // append this token and next token to current photon's token
                },
                TokenType::Photon => {
                    // seperate photon
                },
            }
        }
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

// impl fmt::Display for ArclightSyntaxTree {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         while self.has_next() {
//             if 

//         }
//     }
// }

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