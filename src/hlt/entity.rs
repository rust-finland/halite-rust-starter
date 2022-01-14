use crate::hlt::PlayerId;
use crate::hlt::position::Position;

pub trait Entity {
    fn owner(&self) -> PlayerId;
    fn position(&self) -> Position;
}
