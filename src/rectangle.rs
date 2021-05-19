// assert_eq!などで自分で定義した構造体のインスタンス同士を比較する場合、
// PartialEqトレイトを実装する必要がある
// また、結果を表示するためにDebugトレイトを実装する必要がある
// PartialEq、Debugトレイトは共に導出可能なトレイトなので、
// deriveアトリビュートで指定出来る
#[derive(PartialEq, Debug)]
pub struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle {
            width,
            height
        }
    }
}
