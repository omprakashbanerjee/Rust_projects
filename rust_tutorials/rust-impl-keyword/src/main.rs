/*adding methods to struct using impl */

struct Rectangle{
  width:u32,
  height:u32
}
impl Rectangle{
  fn print_description(&self){
    println!("Rectanle {}x{}", self.width, self.height);
  }
fn is_square(&self) -> bool{
  return self.width == self.height;
}
  
}
fn main() {
   let new_rect = Rectangle{width:10, height:20};
   new_rect.print_description();
   println!("Rectangle is square {} ",new_rect.is_square());

   }
