#[derive(Debug, Clone, Copy)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
