
#[async_std::main]
async fn main() {
  let x = 10;
  
  let rx = &x;

  assert_eq!(rx + 1, 11);

  println!("你: {:?}", desribe_point(-2, 2));
}

fn desribe_point(x: i32, y: i32) -> &'static str {
  use std::cmp::Ordering::*;

  match (x.cmp(&0), y.cmp(&0))  {
    (Equal, Equal) => "at the origin",
    (_, Equal) => "on the x axis",
    (Equal, _) => "on the y axis",
    (Greater, Greater) => "in the first quadrant",
    (Less, Greater) => "in the second quadrant",
    _ => "somewhere else"
  }
}
