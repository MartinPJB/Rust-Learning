// Exercise: Write a code which will check whether _a given integer number is even or odd.
fn oddOrEven(_a:i32) { 
   if _a % 2 == 0 {
      println!("Number {} is even", _a);
   } else {
      println!("Number {} is odd", _a);
   }
}

fn main() {
    oddOrEven(4);
    oddOrEven(9);
}
