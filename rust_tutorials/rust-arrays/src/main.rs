
use std::io;
fn main() {
  //let user_array;
  let numbers:[u8; 7] = [10, 20, 30, 40, 50, 60, 70];
  for n in numbers.iter()
  {
  println!("element is {}",n);
  /*here as we are using iter() fn therefore just mentioning the for loop varibale name will access the array itself*/
  }

/*another way of accessing arrays */
  for n in 0..numbers.len()
  {
  println!("element is {}", numbers[n]);
  /*here iter() fn is not used therefore we have to acces the array like above */
  }

}
