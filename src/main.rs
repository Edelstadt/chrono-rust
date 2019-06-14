use historical_chrono::{Concurrents, MovableFeast};
use historical_chrono::DominicalLetter;
use historical_chrono::Epact;
use historical_chrono::GoldenNumber;
use historical_chrono::SolarCircle;
use historical_chrono::{Chrono, Eastern};

fn main() {
    let year = 2015;
    let chrono = Chrono::new(year, true);
    let solar_circle = chrono.get_solar_circle();
    let dominical_letter_j = chrono.get_dominical_letter_j();
    let dominical_letter_g = chrono.get_dominical_letter_g();
    let concurrents = chrono.get_concurrents();
    let golden_number = chrono.get_golden_number();
    let epact_j = chrono.get_epact_j();
    let epact_g = chrono.get_epact_g();
    let eastern_j = chrono.eastern_j();
    let eastern_g = chrono.eastern_g();
    let septuagesime = chrono.septuagesima();
    let quinquagesina = chrono.quinquagesina();
    let second_sun_lent = chrono.second_sun_lent();

    println!("Year {}", year);
    println!("solar circle {}", solar_circle);
    println!("dom letter j {}", dominical_letter_j);
    println!("dom letter g {}", dominical_letter_g);
    println!("concurrents {}", concurrents);
    println!("golden number {}", golden_number);
    println!("epact j {}", epact_j);
    println!("epact g {}", epact_g);
    println!("eastern j {}", eastern_j);
    println!("eastern g {}", eastern_g);
    println!("septuagesime {}", septuagesime);
    println!("quinquagesina {}", quinquagesina);
    println!("second_sun_lent {}", second_sun_lent);
}
