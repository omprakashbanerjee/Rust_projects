fn main() {
 // let my_vector: Vec<i32> = Vec::new();
  let mut my_vector = vec![1,2,3];
  /* without declaring the vector as mutable we cant modify it */
  println!("{}",my_vector[2]);

  my_vector.push(10); // this will push an element at the end of the vectors
 for i in my_vector.iter(){
println!("{}",i);

}
my_vector.remove(3);
/*if we dont use iter() then my_vector will lose the ownership of its elements */
for i in my_vector.iter(){
println!("{}",i);
}
}

