use std::io;
fn main() {
  println!("Please input your guess");
  let mut guess = String::new();
  io::stdin()
  .read_line(&mut guess)
  .expect("failed to read line");
  let guess: u32 = guess.trim().parse().expect("Please type a number!");
    println!("printing even numbers upto");
 print_numbers_to(guess);
}
    
fn print_numbers_to(num:u32)
{
  for n in 1..num
  {
   if is_even(n){
    println!("{} it is even",n);
  }
    else {
      println!("{} it is odd",n);
    }
  }
}
fn is_even(num:u32) -> bool
{
  return num %2 ==0;
}

