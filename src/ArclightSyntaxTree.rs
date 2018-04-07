use std::fmt;
use std::collections::VecDeque;
use std::cmp::max;

use Photon::Photon;
use Parser::{parse,Tokens};
use Parser::{DEFINED,RETURN};

pub struct ArclightSyntaxTree {
    photons: Vec<Photon>,
    marker: Vec<usize>,
}

impl<'ast> ArclightSyntaxTree {
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
                            break;
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


    pub fn partial_iter(&self) -> ArclightSyntaxTreePartialIter {
        self.partial_iter_from(Some(0usize))
    }

    pub fn partial_iter_from(&self, from: Option<usize>) -> ArclightSyntaxTreePartialIter {
        ArclightSyntaxTreePartialIter {
            ast: self,
            cur: from,
            last: None,
            depth: self.marker_depth(from.unwrap_or(0usize)).unwrap()
        }
    }

    pub fn iter(&self) -> ArclightSyntaxTreeIter {
        self.iter_from(Some(0usize))
    }

    pub fn iter_from(&self, from: Option<usize>) -> ArclightSyntaxTreeIter {
        ArclightSyntaxTreeIter{
            ast: self,
            cur: from,
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

    pub fn up(&'ast mut self) -> Result<&'ast mut ArclightSyntaxTree,AstBuilderError> {
        let current_photon_index = self.marker.pop().unwrap();
        if self.photons.get(current_photon_index).unwrap().up.is_some() {
            self.marker.push(self.photons.get(current_photon_index).unwrap().up.unwrap());
            Ok(self)
        } else {
            self.marker.push(current_photon_index);
            Err(AstBuilderError::MarkerNotFound)
        }
    }

    pub fn down(&'ast mut self) -> Result<&'ast mut ArclightSyntaxTree,AstBuilderError>  {
        let current_photon_index = self.marker.pop().unwrap();
        if self.photons.get(current_photon_index).unwrap().down.is_some() {
            self.marker.push(self.photons.get(current_photon_index).unwrap().down.unwrap());
            Ok(self)
        } else {
            self.marker.push(current_photon_index);
            Err(AstBuilderError::MarkerNotFound)
        }
    }

    pub fn left(&'ast mut self) -> Result<&'ast mut ArclightSyntaxTree,AstBuilderError>  {
        let current_photon_index = self.marker.pop().unwrap();
        if self.photons.get(current_photon_index).unwrap().left.is_some() {
            self.marker.push(self.photons.get(current_photon_index).unwrap().left.unwrap());
            Ok(self)
        } else {
            self.marker.push(current_photon_index);
            Err(AstBuilderError::MarkerNotFound)
        }
    }

    pub fn right(&'ast mut self) -> Result<&'ast mut ArclightSyntaxTree,AstBuilderError>  {
        let current_photon_index = self.marker.pop().unwrap();
        if self.photons.get(current_photon_index).unwrap().right.is_some() {
            self.marker.push(self.photons.get(current_photon_index).unwrap().right.unwrap());
            Ok(self)
        } else {
            self.marker.push(current_photon_index);
            Err(AstBuilderError::MarkerNotFound)
        }
    }

    pub fn marker_token(&mut self) -> &str {
        let current_photon_index = self.marker.pop().unwrap();
        let token_str = self.photons.get(current_photon_index).unwrap().token.as_str();

        self.marker.push(current_photon_index);
        token_str
    }


    fn row_tokens(&mut self) -> String {
        let mut row_tokens = String::new();

        loop {
            if row_tokens.len() > 0 {
                row_tokens.push_str(" ");
            }
            row_tokens.push_str(self.marker_token());

            if self.right().is_err() {
                break;
            }
        }
        row_tokens
    }

    pub fn marker_position(&mut self, new_position: usize) -> Result<&mut ArclightSyntaxTree,AstBuilderError> {
        if new_position < self.photons.len() {
            self.marker.push(new_position);
            Ok(self)
        } else {
            Err(AstBuilderError::MarkerNotFound)
        }
    }
}

#[derive(Debug)]
pub enum AstBuilderError {
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
        if self.cur.is_some() && self.cur.unwrap() < self.ast.len() { // TODO relies on inability to remove photons without full AST rebuild. Memory leak, I guess
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

pub struct ArclightSyntaxTreePartialIter<'ast> {
    ast: &'ast ArclightSyntaxTree,
    cur: Option<usize>,
    last: Option<usize>,
    depth: usize,
}

impl<'ast> ArclightSyntaxTreePartialIter<'ast> {
    pub fn resume(&mut self) -> Option<&'ast Photon> {
        self.cur = self.last; // TODO: figure out how to handle premature resumes. Maybe only allow resume if self.cur=None
        while self.depth <= self.ast.marker_depth(self.cur.unwrap()).unwrap() {
            if self.ast.photons[self.cur.unwrap()].up.is_some() {
                self.cur = self.ast.photons[self.cur.unwrap()].up;
            } else if self.ast.photons[self.cur.unwrap()].left.is_some() {
                self.cur = self.ast.photons[self.cur.unwrap()].left;
            } else if self.ast.photons[self.cur.unwrap()].right.is_some() {
                self.cur = self.ast.photons[self.cur.unwrap()].right;
                let current = &self.ast.photons[self.cur.unwrap()];
                if let Some(down_index) = current.down {
                    self.cur = Some(down_index);
                } else if let Some(right_index) = current.right {
                    self.cur = Some(right_index);
                } else {
                    self.cur = None;
                }
                return Some(current);
            } else {
                return None;
            }
        }
        return None;
    }
}

impl<'ast> Iterator for ArclightSyntaxTreePartialIter<'ast> {
    type Item = &'ast Photon;

    fn next(&mut self) -> Option<Self::Item> { // TODO: need to implement loop as in resume()
        if self.cur.is_some() && self.cur.unwrap() < self.ast.len() {
            if self.last.is_some() {
                self.last = self.cur;
                if let Some(down_index) = self.ast.photons[self.cur.unwrap()].down {
                    self.cur = Some(down_index);
                } else if let Some(right_index) = self.ast.photons[self.cur.unwrap()].right {
                    self.cur = Some(right_index);
                } else {
                    self.cur = None;
                }
            } else {
                self.last = self.cur;
            }
            if self.cur.is_some() {
                Some(&self.ast.photons[self.cur.unwrap()])
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
    fn nested_pass_test() {
        let mut actual = ArclightSyntaxTree::new();
        actual.build_at_marker(parse("a< b< c\n\td<\n\t\te\n\tf").expect("Testing nested_pass_test, actual parse"));

        actual.marker_position(0);
        assert_eq!(actual.marker_token(), "a");
        assert_eq!(actual.down().expect("").marker_token(), "d");
        assert_eq!(actual.down().expect("").marker_token(), "e");
        assert_eq!(actual.up().expect("").up().expect("").right().expect("").marker_token(), "b");
        assert_eq!(actual.down().expect("").marker_token(), "f");
        assert_eq!(actual.up().expect("").right().expect("").marker_token(), "c");
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

    #[test]
    fn partial_iter_test() {
        let mut actual = ArclightSyntaxTree::new();
        actual.build_at_marker(parse("a< b< c\n\td<\n\t\te\n\tf").expect("Testing partial_iter_test, actual parse"));

        let mut partial_iter = actual.partial_iter();

        assert_eq!(partial_iter.next().unwrap().token, "a");
        assert_eq!(partial_iter.next().unwrap().token, "d");
        assert_eq!(partial_iter.next().unwrap().token, "e");
        assert_eq!(partial_iter.next(), None);
        assert_eq!(partial_iter.resume().unwrap().token, "b");
        assert_eq!(partial_iter.next().unwrap().token, "f");
        assert_eq!(partial_iter.next(), None);
        assert_eq!(partial_iter.resume().unwrap().token, "c");
        assert_eq!(partial_iter.next(), None);
        assert_eq!(partial_iter.resume(), None);
    }
}
