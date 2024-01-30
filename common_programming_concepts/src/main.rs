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

    // 3.2 Data Types
    let z: f64 = 2.0; // f64
  println!("The value of z is: {z}");
  let w: f32 = 3.0; // f32
  println!("The value of w is: {w}");

  // addition
  let sum = 5 + 10;

  // subtraction
  let difference = 95.5 - 4.3;

  // multiplication
  let product = 4 * 30;

  // division
  let quotient = 56.7 / 32.2;
  let truncated = -5 / 3; // Results in -1

  // remainder
  let remainder = 43 % 5;

  let t = true;

  let f: bool = false; // with explicit type annotation

  let c = 'z';
  let d: char = 'ℤ'; // with explicit type annotation
  let heart_eyed_cat = '😻';

  let tup: (i32, f64, u8) = (500, 6.4, 1);

  let (e, f, g) = tup;

  println!("The value of f is: {f}");

  let h: (i32, f64, u8) = (500, 6.4, 1);

  let five_hundred = h.0;

  let six_point_four = h.1;

  let one = h.2;

  let i: [i32; 5] = [1, 2, 3, 4, 5];

  let j = [3; 5];
  
  let first = i[0];
  let second = i[1];

}
