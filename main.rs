use std::io;
fn main() {
  println!("Hello World!");
  let mut a: i8 = 16;
  println!("{}",a);
  let mut b = String::new();
  io::stdin().read_line(&mut b) .expect("Failed to read line");
  let mut b: i8 = b.trim().parse().expect("Please input a number");
  if a > b{
	  println!("A is greater");
  }
  else if b > a{
	  println!("B is greater");
  }
  else{
	  println!("They are the same");
  }
}