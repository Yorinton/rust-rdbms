pub struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    pub fn can_hold(&self, other: Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle {
            width,
            height
        }
    }
}