use std::fmt::Display;
use std::ops::Add;

struct Par<T> {
    x: T,
    y: T,
}

impl<T: Display> ToString for Par<T> {
    fn to_string(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}

impl<T> Par<T>
where
    T: Add<Output = T>,
    T: Copy,
{
    fn sestej(&self, other: &Self) -> Par<T> {
        Par {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
