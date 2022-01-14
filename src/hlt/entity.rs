use crate::hlt::position::Position;
use crate::hlt::PlayerId;

pub trait Entity {
    fn owner(&self) -> PlayerId;
    fn position(&self) -> Position;
}
