pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub const ZERO: Self = Self { x: 0, y: 0 };

    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
