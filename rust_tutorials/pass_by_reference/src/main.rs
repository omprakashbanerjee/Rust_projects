struct Color {
  red: u8,
  blue: u8,
  green: u8
}
fn main() {
  let blue= Color{red: 0,blue: 255,green: 0 };
  print_color(&blue); 
  /*print_color(blue); this will also work same as above but scope of the blue will be transfered to this function
  and we wont be able to access again after this, therefore we need to call by reference only */
}
fn print_color(c: &Color) {
  println!("Red: {} \nBlue: {} \nGreen: {} ",c.red, c.blue, c.green);
}
