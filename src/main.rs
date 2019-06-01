use historical_chrono::Concurrents;
use historical_chrono::DominicalLetter;
use historical_chrono::Epact;
use historical_chrono::GoldenNumber;
use historical_chrono::SolarCircle;
use historical_chrono::{Chrono, Eastern};

fn main() {
    let chrono = Chrono::new(2015, true);
    let solar_circle = chrono.get_solar_circle();
    let dominical_letter_j = chrono.get_dominical_letter_j();
    let dominical_letter_g = chrono.get_dominical_letter_g();
    let concurrents = chrono.get_concurrents();
    let golden_number = chrono.get_golden_number();
    let epact_j = chrono.get_epact_j();
    let epact_g = chrono.get_epact_g();
    let eastern_j = chrono.eastern_j();

    println!("solar circle {}", solar_circle);
    println!("dom letter j {}", dominical_letter_j);
    println!("dom letter g {}", dominical_letter_g);
    println!("concurrents {}", concurrents);
    println!("golden number {}", golden_number);
    println!("epact j {}", epact_j);
    println!("epact g {}", epact_g);
    println!("eastern j {}", eastern_j);
}