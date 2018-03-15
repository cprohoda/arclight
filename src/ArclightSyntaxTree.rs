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
            photons: vec![Photon::new("".to_string())],
            marker: vec![0],
        }
    }

    pub fn build_at_marker(&mut self, tokens: Tokens) -> Result<(),AstBuilderError> {
        use Parser::TokenType;

        let current_photon = self.marker.pop().unwrap();
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
                    self.photons[current_photon].push_to_token(DEFINED.to_string());
                    self.marker.push(current_photon);
                },
                TokenType::Return => {
                    self.photons[current_photon].push_to_token(RETURN.to_string());
                    self.marker.push(current_photon);
                },
                TokenType::Photon => {
                    self.photons[current_photon].push_to_token(token.token_str().to_string());
                    self.marker.push(current_photon);
                },
            }
        }
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.photons.len()
    }

    pub fn iter(&self) -> ArclightSyntaxTreeIter {
        self.iter_from(0)
    }

    pub fn iter_from(&self, from: usize) -> ArclightSyntaxTreeIter {
        ArclightSyntaxTreeIter{
            ast: self,
            cur: from,
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

pub struct ArclightSyntaxTreeIter<'ast> {
    ast: &'ast ArclightSyntaxTree,
    cur: usize,
}

impl<'ast> Iterator for ArclightSyntaxTreeIter<'ast> {
    type Item = &'ast Photon;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur < self.ast.len() { // relies on inability to remove photons without full AST rebuild
            if let Some(down_index) = self.ast.photons[self.cur].down {
                self.cur = down_index;
                Some(&self.ast.photons[down_index])
            } else if let Some(right_index) = self.ast.photons[self.cur].right {
                self.cur = right_index;
                Some(&self.ast.photons[right_index])
            } else if let Some(up_index) = self.ast.photons[self.cur].up {
                if let Some(up_right_index) = self.ast.photons[up_index].right {
                    self.cur = up_right_index;
                    Some(&self.ast.photons[up_right_index])
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

mod tests {
    use ArclightSyntaxTree::ArclightSyntaxTree;
    use Photon::Photon;
    use Parser::parse;

    #[test]
    fn defined_build_test() {
        let mut expected = ArclightSyntaxTree::new();
        expected.photons[0].push_to_token("a.b".to_string());

        let mut actual = ArclightSyntaxTree::new();
        actual.build_at_marker(parse("a.b").expect("Testing defined_build_test, actual parse"));

        assert_eq!(expected, actual);
    }
}
