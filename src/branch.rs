/// Basic source code data structure for arclight
/// Recursive data structure representing a given token and references to its super/previous and sub/next branches
use std::boxed::Box;
use core::ptr::Shared;
use core::marker::PhantomData;
use std::cell::RefCell;

pub struct ArclightSyntaxTree<T> {
	head: Option<Shared<Branch<T>>>,
	// to add: a hashmap of tokens
	len: usize,
	marker: PhantomData<Box<Branch<T>>>,
}

impl<T> ArclightSyntaxTree<T> {
	fn Iter(marker: PhantomData<Box<Branch<T>>>) {
		if some(Self.marker.d) {
			Self.marker = Self.marker.d;
		} else if some(Self.marker.r) {
			Self.marker = Self.marker.r;
		} else {
				while some(Self.marker.l) {
				Self.marker = Self.marker.l.marker;
			}
			Self.marker = Self.marker.u.r.marker;
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