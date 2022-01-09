use std::fmt::{ Debug, Formatter };

struct Point {
    x: i32,
    y: i32,
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
    }
}

#[async_std::main]
async fn main() {
    let origin = Point { x: 1, y: 2 };
    println!("the origin = {:?}", origin);
}
