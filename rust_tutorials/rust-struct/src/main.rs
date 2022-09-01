struct Color {
  red:u8,
  blue:u8,
  green:u8
  /*members of a struct */
}

/*struct of tuples */
struct Animals(u8,i32,u8);

fn main() {
  let bg = Color{red:255, blue:200,green:100};
  /*we can not change the value of a struct unless is mutable
  bg.red = 40; wont work */
  println!("red is {} \n blue is {} \n green is {}", bg.red, bg.blue,bg.green);
let cow = Animals(4,100,200);
println!("cow is {} and {} and {}", cow.0, cow.1, cow.2);

}
