use std::fmt;
use std::collections::VecDeque;
use std::cmp::max;

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

        let mut current_photon_index = self.marker.pop().unwrap();

        for token in tokens.iter() {
            match *token.token_type() {
                TokenType::Control => {
                    let target_depth = token.token_str().to_string().replace("\n", "").chars().count();
                    // check marked locations for an empty photon at the expected depth
                    for marker in &self.marker {
                        if self.photons[*marker].token == "".to_string() && self.marker_depth(*marker).unwrap() == target_depth { // TODO: Also need to check if it's correctly placed
                            current_photon_index = *marker;
                        }
                    }
                },
                TokenType::Pass => {
                    self.photons.push(Photon::new("".to_string()));
                    let last_photon_index = self.photons.len() - 1;
                    self.photons[last_photon_index].up = Some(current_photon_index);
                    self.photons[current_photon_index].down = Some(last_photon_index);
                    self.marker.push(last_photon_index);
                },
                TokenType::Defined => {
                    self.photons[current_photon_index].push_to_token(DEFINED.to_string());
                    self.marker.push(current_photon_index);
                },
                TokenType::Return => {
                    self.photons[current_photon_index].push_to_token(RETURN.to_string());
                    self.marker.push(current_photon_index);
                },
                TokenType::Photon => {
                    if self.photons[current_photon_index].token == "".to_string() {
                        self.photons[current_photon_index].push_to_token(token.token_str().to_string());
                    } else {
                        self.photons.push(Photon::new("".to_string()));
                        let last_photon_index = self.photons.len() - 1;
                        self.photons[last_photon_index].left = Some(current_photon_index);
                        self.photons[current_photon_index].right = Some(last_photon_index);
                        current_photon_index = last_photon_index;
                        self.photons[current_photon_index].push_to_token(token.token_str().to_string());
                    }
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
            cur: Some(from),
        }
    }

    pub fn marker_depth(&self, marker: usize) -> Result<usize,AstBuilderError> {
        let mut marker_depth = 0usize;
        if self.photons.get(marker).is_none() {
            return Err(AstBuilderError::MarkerNotFound);
        }
        let mut index = marker;
        loop {
            if self.photons[index].left.is_some() {
                index = self.photons[index].left.unwrap();
            } else if self.photons[index].up.is_some() {
                index = self.photons[index].up.unwrap();
                marker_depth += 1;
            } else {
                break;
            }
        }
        Ok(marker_depth)
    }

    // fn to_alf(filename: &str) -> Result<> {

    // }

    // fn from_alf(filename: &str) -> Result<ArclightSyntaxTree,E> {

    // }
}

#[derive(Debug)]
enum AstBuilderError {
    Unknown,
    UnmatchedDepth,
    MarkerNotFound,
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
    cur: Option<usize>,
}

impl<'ast> Iterator for ArclightSyntaxTreeIter<'ast> {
    type Item = &'ast Photon;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur.is_some() && self.cur.unwrap() < self.ast.len() { // relies on inability to remove photons without full AST rebuild
            let current = &self.ast.photons[self.cur.unwrap()];
            if let Some(down_index) = current.down {
                self.cur = Some(down_index);
            } else if let Some(right_index) = current.right {
                self.cur = Some(right_index);
            } else if let Some(up_index) = current.up {
                if let Some(up_right_index) = self.ast.photons[up_index].right {
                    self.cur = Some(up_right_index);
                } else {
                    self.cur = None;
                }
            } else {
                self.cur = None;
            }
            Some(current)
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

    #[test]
    fn pass_build_test() {
        let mut actual = ArclightSyntaxTree::new();
        actual.build_at_marker(parse("a<\n\tb").expect("Testing pass_build_test, actual parse"));

        assert_eq!(actual.photons[actual.photons[0].down.unwrap()].token, "b".to_string());
    }

    #[test]
    fn pass_delayed_build_test() {
        let mut actual = ArclightSyntaxTree::new();
        actual.build_at_marker(parse("a< b c\n\td").expect("Testing pass_delayed_build_test, actual parse")); // TODO: this creates a space after the pass. Still works, but should fix the parse algorithm

        assert_eq!(actual.photons[actual.photons[0].down.unwrap()].token, "d".to_string());
        assert_eq!(actual.photons[actual.photons[0].right.unwrap()].token, "b".to_string());
        assert_eq!(actual.photons[actual.photons[actual.photons[0].right.unwrap()].right.unwrap()].token, "c".to_string());
    }

    #[test]
    fn ast_iter_test() {
        let mut actual = ArclightSyntaxTree::new();
        actual.build_at_marker(parse("a< b c\n\td").expect("Testing pass_delayed_build_test, actual parse"));

        let mut actual_iter = actual.iter();

        assert_eq!(actual_iter.next().unwrap().token, "a");
        assert_eq!(actual_iter.next().unwrap().token, "d");
        assert_eq!(actual_iter.next().unwrap().token, "b");
        assert_eq!(actual_iter.next().unwrap().token, "c");
        assert_eq!(actual_iter.next(), None);
    }
}
