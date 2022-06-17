use std::fs::File;
use std::io::prelude::*;
/* prelude module helps us to read and write operations to file  */


fn main() {
  let mut file =File::open("textfile.txt").expect("Can't open file");
  let mut contents = String::new();
  file.read_to_string(&mut contents)
  .expect("oops cant read");
  println!("{}", contents);
    
}
