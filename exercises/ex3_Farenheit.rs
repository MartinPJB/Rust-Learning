// Exercise: Create a function that takes in a temperature in Fahrenheit and returns the equivalent temperature in Celsius.
fn to_celsius(farenheit: i32) -> f64 {
    return (farenheit - 32) as f64 / 1.8;
}

fn main() {
    let farenheit_to_celsius = to_celsius(145);
    print!("145°F = {}°C", farenheit_to_celsius);
}
