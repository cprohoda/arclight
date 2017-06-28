/// Basic source code data structure for arclight
/// Recursive data structure representing a given token and references to its super/previous and sub/next branches
use std::boxed::Box;
use core::ptr::Shared;
use core::marker::PhantomData;
use std::cell::RefCell;

// TODO: consider ARC or a non-hashmap backend

pub struct ArclightSyntaxTree<T> {
	head: Option<Shared<Branch<T>>>,
	// TODO: add a hashmap of tokens
	len: usize,
	marker: PhantomData<Box<Branch<T>>>,
}

enum IterError {
	TreeEnd,
}

type IterResult = Result(Ok(),IterError);

impl<T> ArclightSyntaxTree<T> {
	fn Iter(&mut self) -> IterResult {
		if some(Self.marker.d) {
			Self.marker = Self.marker.d;
			Ok()
		} else if some(Self.marker.r) {
			Self.marker = Self.marker.r;
			Ok()
		} else {
			while some(Self.marker.l) {
				Self.marker = Self.marker.l;
			}
			if some(self.marker.u.r) {
				Self.marker = Self.marker.u.r;
				Ok()
			} else {
				IterError::TreeEnd
			}
		}
	}
}

struct Branch<T> {
	u: Option<Weak<RefCell<Branch<T>>>>,
	l: Option<Weak<RefCell<Branch<T>>>>,
	d: Option<Rc<RefCell<Branch<T>>>>,
	r: Option<Rc<RefCell<Branch<T>>>>,
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

	fn LinkR(r: Branch) -> Self {
		r.l = Self;
		r.r = Self.r;
		Self.r = r;
	}

	fn LinkD(d: Branch) -> Self {
		d.u = Self;
		d.d = Self.d;
		Self.d = d;
	}
}