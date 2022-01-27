
#[async_std::main]
async fn main() {
  let x = 10;
  
  let rx = &x;

  assert_eq!(rx + 1, 11);

  let account = Account { name: "maotingfeng".to_string(), language: "chinese".to_string() };

  let a = match account {
    Account { ref name, .. } => {
      greet(&name);
      2
    },
    _ => 1,
  };
  let bbb = account;
}

fn greet(name: &str) -> i32 {
  println!("{:?}", name);
  return 2;
}

#[derive(Debug)]
struct Account {
  name: String,
  language: String,
}

