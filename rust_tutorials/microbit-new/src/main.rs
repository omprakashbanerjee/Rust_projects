const MAX_NUMBER:u8 = 25;
/*constants can be accessed but not changed*/
/*constants should be declared with the data types*/
fn main() {
    let x =45;// defining unmutable variables
   // println!("The val x is {}",x); // printing it
    /*x=60;
      println!("The val x is {}",x); // printing it
      */ // updating an unmutable varibale is not allowed
  let mut y =45;// defining mutable variables
      // println!("The val y is {}",y); // printing it

      // y= 60;// updating an mutable

      // println!("The val y is {}",y); // printing it
   let mut a:i64 =10; // signed integer 64 bit, similary i32 for signer inter 32 bit
//   let mut b:u64 =20; // unsigned integer
   let mut _c:f32=20.12;// float 32 bit(in case this varibale is not mutated later in the code then underscore is given when declared)
//   let mut d:bool=true;// booleans

   /*if we declare a varibale as mutable and not update in the code then it will give warning 
   in this case we can declare any varibale by just putting a underscore before the variable*/

   /*if else statements*/
  //  if a>30 {
  //    println!("a is smaller than 30")
  //   }
  //   else {
  //     println!("if condition didnt match")
  //   }


// Functions in Rust
// print_numbers_to(15);
let beta = 20;
{
  let alpha = 10;
  println!("{} is alpha and {} is beta", alpha, beta);
  /*This is a block of codwe which has access of this block as well as outer main block */
  /*varibales declared in this scope will not be available to other block outside here  */
  let y = 20; // on top of the code it was defined as 45 but here we are defining it as 20
  println!("shadow value of y inside the code block is {}",y);

}
println!("un shadow value of y is now {}",y);
/*shadowing is when we re define a varibale inside a block but outside the block we want our old value of the varible  */
 
let yr = &y;
println!("value of y is {}  using its reference", yr);

/*to update the value of a varibale using its reference can be done if the reference is declared as mutable  */
let yrr = &mut y;
*yrr +=1;
println!("value of y updated to  {} using its reference",yrr)
}


// fn print_numbers_to(num:u32)
// {
//   for n in 1..num
//   {
//    if is_even(n){
//     println!("{} it is even",n);
//   }
//     else {
//       println!("{} it is odd",n);
//     }
//   }
// }
// fn is_even(num:u32) -> bool{
//   return num %2 ==0;
// }
