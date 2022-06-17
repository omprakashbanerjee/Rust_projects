#![allow(dead_code)]

enum Days {
  Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday
}
impl Days{
  fn is_weekend(&self) ->bool{
    match self {
      &Days::Saturday | &Days::Sunday => return true,
      _ => return false
    }
  }
}

fn main() {
  let d= Days::Monday;
  println!("is it a weekend {}", d.is_weekend());
   
}
