/// Basic source code data structure for arclight
/// Recursive data structure representing a given token and references to its super/previous and sub/next branches
use std::boxed::Box;
use core::ptr::Shared;
use core::marker::PhantomData;

pub struct ArclightSyntaxTree<T> {
	head: Option<Shared<Branch<T>>>,
	// to add: a hashmap of tokens
	len: usize,
	marker: PhantomData<Box<Branch<T>>>,
}

impl<T> ArclightSyntaxTree<T> {
	fn Traverse(marker: PhantomData<Box<Branch<T>>>) {
		if some(Self.marker.d) {
			Self.marker = Self.marker.d;
		} else if some(Self.marker.r) {
			Self.marker = Self.marker.r;
		} else {
				while some(Self.marker.l) {
				Self.marker = Self.marker.l;
			}
			Self.marker = Self.marker.u.r;
		}
	}
}

pub struct Iter<'a, T: 'a> {
}

struct Branch<T> {
	u: Option<Branch<T>>,
	d: Option<Branch<T>>,
	l: Option<Branch<T>>,
	r: Option<Branch<T>>,
	token: T,
}

impl<T> Branch<T> {
	fn New(token: T) -> Self {
		Branch {
			token: token,
			u: None,
			d: None,
			l: None,
			r: None,
		}
	}
}