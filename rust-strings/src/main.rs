fn main() {
   let my_name = "om prakash banerjee";
   println!("{}", my_name);
   println!("lenght of my name {}",my_name.len());

   let my_string = String::from("hello world");
   println!("Length is {}", my_string.len());
   println!("string is empty {}", my_string.is_empty());

   /*to split whitespace */
for token in my_name.split_whitespace(){
  println!("{}", token);
}

println!("Does my_name contains 'om' {} ",my_name.contains("om"));

}
