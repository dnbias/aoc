#![allow(dead_code)]
mod diagnostic;
use diagnostic::calculate_diagnostic;
use diagnostic::ratings;

fn main() {
    // power_consumption();
    life_support_rating();
}

fn power_consumption() {
    let (gamma,epsilon) = calculate_diagnostic("input").unwrap();
    let gamma = gamma as u64;
    let epsilon = epsilon as u64;
    let power_consumption = gamma * epsilon;
    println!("Power Consumption: {}", power_consumption);
}

fn life_support_rating() {
    let (ox,co) = ratings("input").unwrap();
    println!("Life Support Rating: {}", ox*co);
}
