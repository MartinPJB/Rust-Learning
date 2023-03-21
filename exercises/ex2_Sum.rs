// Exercise: Create a function that takes in two numbers and returns their sum.
fn add(number1: i32, number2: i32) -> i32 {
    return number1 + number2;
}

fn main() {
    let sum_of_two_and_two = add(2, 2);
    print!("Sum of 2 and 2 = {}", sum_of_two_and_two);
}
