
#[async_std::main]
async fn main() {
  let x = 10;
  
  let rx = &x;

  assert_eq!(rx + 1, 11);

  println!("x={:?}", match_point(Point { x: 0, y: 2 }));
}

struct Point {
  x: i32,
  y: i32,
}

fn match_point(bollon: Point) -> i32 {
  match bollon {
    Point { x: 0, y: height } => height,
    Point { x, y } => x + y,
  }
}
