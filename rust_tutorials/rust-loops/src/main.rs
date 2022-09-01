fn main() {
    println!("Hello, world!");

    let mut y: u8 = 45;
//  loop keyword
loop
 {
    y+=10;
    println!("value of y is {}", y);
    if y>100
    {
        break;
    }
 }
 y = 40;
  while y<=100{
    if y%2 ==0 { 
    println!("y = {}", y);
    
    }
    y+=1;
  }

//  for loop 
for i in 1..10 {
  println!("the number is {}", i);
}
let range = 10..15;

for b in range {
  println!("the second number is {}", b);
}
// Vectors
let animals = vec!["dog","cat", "cow"];
 for a in animals.iter()
    {
     println!("the animals are {}", a)
    }
// if we dont use .iter() in the for loop then ownership of animals will be transfered to for loop
// and we wont be able to access the animals after the loop
    for (index,a) in animals.iter().enumerate()
    {
    println!("{} animal is  {}", index,a);
    }

 }