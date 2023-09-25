use std::io::{self, Write};

#[derive(Debug)]
struct Rectangle(u32, u32);

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self(width, height)
    }

    fn create_square(size: u32) -> Self {
        Self(size, size)
    }

    fn area(&self) -> u32 {
        self.0 * self.1
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        (other.0 < self.0 && other.1 < self.1) || (other.0 < self.1 && other.1 < self.0)
    }
}

#[allow(dead_code)]
fn read_u32(prompt: &str) -> u32 {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input
        .trim()
        .parse()
        .expect("You are likely converting an non-digit character to u32")
}

fn main() {
    let rect1 = Rectangle::new(30, 50);
    let rect2 = Rectangle::new(10, 40);
    let rect3 = Rectangle::new(60, 45);
    let square = Rectangle::create_square(70);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can square hold rect3? {}", square.can_hold(&rect3));
    println!("Can rect3 hold square? {}", rect3.can_hold(&square));
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("Rectangle 1: {:?}", rect1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle() {
        let rect = Rectangle::new(30, 50);
        let square = Rectangle::create_square(50);

        assert_eq!(rect.0, 30);
        assert_eq!(rect.1, 50);
        assert_eq!(square.0, 50);
        assert_eq!(square.0, square.1);
    }

    #[test]
    fn test_can_hold() {
        let rect1 = Rectangle::new(30, 50);
        let rect2 = Rectangle::new(10, 40);
        let square = Rectangle::create_square(50);

        assert_eq!(rect1.can_hold(&rect2), true);
        assert_eq!(rect1.can_hold(&square), false);
        assert_eq!(rect2.can_hold(&rect1), false);
        assert_eq!(square.can_hold(&rect2), true);
        assert_eq!(square.can_hold(&rect1), false);
    }
}
