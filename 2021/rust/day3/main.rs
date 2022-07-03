mod diagnostic;
use diagnostic::calculate_diagnostic;

fn main() {
    let (gamma,epsilon) = calculate_diagnostic("input").unwrap();
    let power_consumption = gamma * epsilon;
    println!("Power Consumption: {}", power_consumption);
}
