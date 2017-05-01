/// Basic source code data structure for arclight
/// Recursive data structure representing a given token and references to its super/previous and sub/next branches
use std::boxed::Box;

pub struct ArclightSyntaxTree<T> {
	head: Option<Branch<T>>,
	// to add: a hashmap of tokens
	len: usize,
}

struct Branch<T> {
	u: Option<Branch<T>>,
	d: Option<Branch<T>>,
	l: Option<Branch<T>>,
	r: Option<Branch<T>>,
	token: T,
}

impl<T> Branch<T> {
	fn new(token: T) -> Self {
		Branch {
			token: token,
			u: None,
			d: None,
			l: None,
			r: None,
		}
	}
}