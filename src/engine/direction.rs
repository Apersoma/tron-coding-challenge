#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    PositiveY,
    NegativeY,
    PositiveX,
    NegativeX,
}
impl Direction {
    pub fn all() -> impl Iterator<Item = Direction> {
        Self::all_slice().into_iter()
    }
    pub const fn all_slice() -> [Self; 4] {
        [
            Direction::PositiveX,
            Direction::PositiveY,
            Direction::NegativeX,
            Direction::NegativeY,
        ]
    }
    pub const fn up() -> Self {
        Direction::PositiveY
    }
    pub const fn down() -> Self {
        Direction::NegativeY
    }
    pub const fn left() -> Self {
        Direction::NegativeX
    }
    pub const fn right() -> Self {
        Direction::PositiveX
    }

    pub const fn left_of(&self) -> Self{
        match self {
            Direction::PositiveY => Direction::NegativeX,
            Direction::NegativeY => Direction::PositiveX,
            Direction::PositiveX => Direction::PositiveY,
            Direction::NegativeX => Direction::NegativeY,
        }
    }
    pub const fn right_of(&self) -> Self{
        match self {
            Direction::PositiveY => Direction::PositiveX,
            Direction::NegativeY => Direction::NegativeX,
            Direction::PositiveX => Direction::NegativeY,
            Direction::NegativeX => Direction::PositiveY,
        }
    }
}