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

  // 3.3 Functions
    another_function(5);

    print_labeled_measurement(5, 'h');

    // statement
    let s = 6;

    // expression
    let e = {
        let e1 = 3;
        e1 + 1
    };

    println!("The value of e is: {e}");

    let f = five();

    println!("The value of f is: {f}");

    let po = plus_one(5);

    println!("The value of po is: {po}");

    // 3.4 Control Flow
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut num = 3;

    while num != 0 {
        println!("{num}!");

        num -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn another_function(x: i32) {
    println!("The value of x in another_function is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement in print_labeled_measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}