use std::io;

fn main() {
  let mut input = String::new();
  println!("please give inputs");
  match io::stdin().read_line(&mut input)
  {
    Ok(_) =>{
      println!("your input is {} ", input.to_uppercase());
    },
    Err(e) => println!("error occured{}", e)
  }
}
   
