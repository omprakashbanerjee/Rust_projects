enum Direction{ Up, Down, Left , Right }


fn main() {
    
/* enum */
/* enum needs to be defined before the main fn therefore check top of the code to see enumerations */
let my_Direction:Direction = Direction :: Up;
/* single colon is used for declaring type and double colon is used to access the varible */
match my_Direction {
  /*match command is similar to switch statement */
  Direction::Up => println!("We are Up \n"),
  Direction::Down => println!("We are Down \n"),
  Direction::Left => println!("We are Left \n"),
  Direction::Right => println!("We are Right \n"),
}
// tuples
println!(" Creating and accessing tuples \n ");

let tup1 = (10,20,30,40,50); // a tuple of integers
println!("{}",tup1.4);
let tup2 = (1,"hello",'a',40.2,true);/*if we write a instead of 'a' then it will print the hex value of a*/
println!("{}",tup2.2);

println!(" nested tuple");
let tup3 = (1,"hello",'a',40.2,true,(1,2,3));
println!("nested yuple vsrible is {}",(tup3.5).2);
/*to access tuple varibles and re assign them into separate variables*/
let (j,k,l,m,n) = tup2;
println!("j is {}",j);
println!("k is {}",k);
println!("l is {}",l);
println!("m is {}",m);
println!("n is {}",n);

}