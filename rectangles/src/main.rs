#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let s = Rectangle::square(3);
    println!("width of s is {}.\nheight of s is {}.", s.width, s.height);
}
