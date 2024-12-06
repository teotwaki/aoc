#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Hash)]
pub enum Direction {
    #[default]
    Up,
    Right,
    Down,
    Left,
}
