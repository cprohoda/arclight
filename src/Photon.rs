#[derive(PartialEq,Debug)]
pub struct Photon { // data structure is a linked list of these elements
    down: Option<usize>,
    up: Option<usize>,
    right: Option<usize>,
    left: Option<usize>,
    token: Option<String>,
}

impl Photon {
    fn new(token: Option<String>) -> Photon {
        Photon{
            down: None,
            up: None,
            right: None,
            left: None,
            token: token,
        }
    }
}


