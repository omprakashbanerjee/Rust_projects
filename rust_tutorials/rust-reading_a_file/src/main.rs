use std::fs::File;
use std::io::prelude::*;
/* prelude module helps us to read and write operations to file  */


fn main() {

  /*reading a file */
  let mut file =File::open("textfile.txt").expect("Can't open file");
  let mut contents = String::new();
  file.read_to_string(&mut contents)
  .expect("oops cant read");
  println!("{}", contents);

/*creating a file  */
  let mut file = File::create("hello.txt").expect("Can't create file");
  file .write_all(b"file created succeefully").expect("oops cant write");


  /*Rad the file created */
  let mut file = File ::open("hello.txt").expect("Can't open file");
  let mut contents = String::new();
  file.read_to_string(&mut contents).expect("cant read the hello file");
  println!("hello file data is {}", contents);

    
}
