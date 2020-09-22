use std::io;

fn main() {
    println!("Enter your weight (kg): ");
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let weight: f32 = input.trim().parse().unwrap();
    println!("Your mars weight is: {}", calculate_mars_weight(&weight));
}

fn calculate_mars_weight(w: &f32) -> f32 {
    (w / 9.81) * 3.711
}
