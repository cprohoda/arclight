#[derive(PartialEq,Debug)]
struct Element { // data structure is a linked list of these elements
    down: Option<usize>,
    up: Option<usize>,
    right: Option<usize>,
    left: Option<usize>,
    token: String,
}

impl Element {
    fn new(token: String) -> Self {
        Element{
            down: None,
            up: None,
            right: None,
            left: None,
            token: token,
        }
    }
}


