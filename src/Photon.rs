#[derive(PartialEq,Debug)]
pub struct Photon {
    pub down: Option<usize>,
    pub up: Option<usize>,
    pub right: Option<usize>,
    pub left: Option<usize>,
    pub token: String,
}

impl Photon {
    pub fn new(token: String) -> Photon {
        Photon{
            down: None,
            up: None,
            right: None,
            left: None,
            token: token,
        }
    }

    pub fn push_to_token(&mut self, partial_token: String) {
        self.token.push_str(partial_token.as_str());
    }
}


