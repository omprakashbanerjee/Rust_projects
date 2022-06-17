struct Person{
  name:String,
  age:u8
}

impl ToString for Person {
  fn to_string(&self) -> String {
    return format!("my name is {} and i'm {}", self.name , self.age);
  }
}
fn main() {
   let om = Person{name :String ::from("Om"), age:27};
   println!("{}", om.to_string());
}
