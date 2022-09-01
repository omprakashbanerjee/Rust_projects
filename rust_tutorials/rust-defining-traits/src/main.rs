
/* trait is like set of certain set of rules or requirements that a object or struct must have 
in order to have that trait  */

struct Person{
  name:String,
  age:u8
}

trait HasVoiceBox{
  fn speak(&self);
  fn can_speak(&self)-> bool;
}
impl HasVoiceBox for Person{
  fn speak(&self){
    println!("my name is {}", self.name);
  }

 fn can_speak(&self)-> bool{
   return self.age >=3;
 }
}
fn main() {
    let person = Person{
      name :String::from("om"),
      age : 27
    };
   person.speak();
    println!("my name is {} & i can speak {}", person.name, person.can_speak());

}
