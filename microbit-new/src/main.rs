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
for i in 1..10 {
  println!("the number is {}", i);
}
let range = 10..15;

for b in range {
  println!("the second number is {}", b);
}

let animals = vec!["dog","cat", "cow"];
for a in animals.iter(){
  println!("the second number is {}", a)
}
// if we dont use .iter() in the for loop then ownership of animals will be transfered to for loop
// and we wont be able to access the animals after the loop

}
