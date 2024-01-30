const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
  // 3.1 Variables and Mutability
  let mut x = 5;
  println!("The value of x is: {x}");
  x = 6;
  println!("The value of x is: {x}");

  println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");
  
  let y = 5;
  let y = y + 1;
  {
      let y = y * 2;
      println!("The value of y in the inner scope is: {y}");
  }
  println!("The value of y is: {y}");

  let spaces = "   ";
  println!("The value of spaces is: {spaces}");
  let spaces = spaces.len();
  println!("The value of spaces is: {spaces}");
}
