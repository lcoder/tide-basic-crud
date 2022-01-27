use std::io::Write;

#[async_std::main]
async fn main() {
  let mut bytes: Vec<u8> = vec![];

  say_hello(&mut bytes);

  println!("list={:?}", bytes);
}


fn say_hello(out: &mut dyn Write) {
  out.write_all(b"hello world\n");
  out.flush();
}
