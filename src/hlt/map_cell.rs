use crate::hlt::position::Position;
use crate::hlt::DropoffId;
use crate::hlt::PlayerId;

pub struct MapCell {
    pub position: Position,
    pub halite: usize,
    pub structure: Structure,
}

#[derive(Eq, PartialEq)]
pub enum Structure {
    None,
    Dropoff(DropoffId),
    Shipyard(PlayerId),
}

impl Structure {
    pub fn is_some(&self) -> bool {
        match *self {
            Structure::None => false,
            _ => true,
        }
    }

    pub fn is_none(&self) -> bool {
        !self.is_some()
    }
}
