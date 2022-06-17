const MAX_NUMBER:u8 = 25;
/*constants can be accessed but not changed*/
/*constants should be declared with the data types*/
enum Direction{ Up, Down, Left , Right }
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
// loop keyword
// loop{
  // y+=10;
   // if y>100{
   //   break;
   // }
   // println!("value of y is {}", y);
// // }
//   while y<=100{
//     if y%2 ==0 { 
//     println!("y = {}", y);
    
//     }
//     y+=1;
//   }

// for loop 
// for i in 1..10 {
//   println!("the number is {}", i);
// }
// let range = 10..15;

// for b in range {
//   println!("the second number is {}", b);
// }

// let animals = vec!["dog","cat", "cow"];
// for a in animals.iter(){
//   println!("the second number is {}", a)
// }
// // if we dont use .iter() in the for loop then ownership of animals will be transfered to for loop
// // and we wont be able to access the animals after the loop
// for (index,a) in animals.iter().enumerate(){
//   println!("{} animal is  {}", index,a);
// }

// enum 
// enum needs to be defined before the main fn therefore check top of the code to see enumerations
// let myDirection:Direction = Direction :: Up;
// /**single colon is used for declaring type and double colon is used to access the varible */
// match myDirection {
//   /*match command is similar to switch statement */
//   Direction::Up => println!("We are Up"),
//   Direction::Down => println!("We are Down"),
//   Direction::Left => println!("We are Left"),
//   Direction::Right => println!("We are Right"),
// }
// tuples

// let tup1 = (10,20,30,40,50); // a tuple of integers
// println!("{}",tup1.4);
// let tup2 = (1,"hello",'a',40.2,true);/*if we write a instead of 'a' then it will print the hex value of a*/
// println!("{}",tup2.2);
// /*nested tuple*/
// let tup3 = (1,"hello",'a',40.2,true,(1,2,3));
// println!("nested yuple vsrible is {}",(tup3.5).2);
// /*to access tuple varibles and re assign them into separate variables*/
// let (j,k,l,m,n) = tup2;
// println!("j is {}",j);
// println!("k is {}",k);
// println!("l is {}",l);
// println!("m is {}",m);
// println!("n is {}",n);


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
