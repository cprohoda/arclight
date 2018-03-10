#[derive(PartialEq,Debug)]
pub struct Photon {
    down: Option<usize>,
    up: Option<usize>,
    right: Option<usize>,
    left: Option<usize>,
    token: Option<String>,
}

impl Photon {
    pub fn new(token: Option<String>) -> Photon {
        Photon{
            down: None,
            up: None,
            right: None,
            left: None,
            token: token,
        }
    }
}


