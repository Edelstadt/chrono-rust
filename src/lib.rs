extern crate chrono;

use chrono::NaiveDate;
use std::collections::HashMap;

const SIMPLE_ARRAY: [&str; 7] = ["A", "B", "C", "D", "E", "F", "G"];
const DOUBLE_ARRAY: [&str; 7] = ["GF", "BA", "DC", "FE", "AG", "CB", "ED"];

fn generate_d_map() -> HashMap<u8, String> {
    let dom_let_array = [
        "GF", "E", "D", "C", "BA", "G", "F", "E", "DC", "B", "A", "G", "FE", "D", "C", "B", "AG",
        "F", "E", "D", "CB", "A", "G", "F", "ED", "C", "B", "A",
    ];
    let mut map = HashMap::new();
    for (i, letter) in dom_let_array.iter().enumerate() {
        map.insert((i + 1) as u8, letter.to_string());
    }
    map
}

fn generate_d_g_map(year: i16) -> HashMap<String, String> {
    fn tt(shift_count: usize) -> HashMap<String, String> {
        let mut copy_simple = SIMPLE_ARRAY;
        let copy_simple_2 = SIMPLE_ARRAY;
        let mut copy_double = DOUBLE_ARRAY;
        let copy_double_2 = DOUBLE_ARRAY;
        copy_simple.rotate_right(shift_count);
        copy_double.rotate_right(shift_count);
        let result_vec = [copy_simple, copy_double]
            .concat()
            .iter()
            .map(|&x| String::from(x) as String)
            .collect::<Vec<String>>();
        let result_vec_2 = [copy_simple_2, copy_double_2]
            .concat()
            .iter()
            .map(|&x| String::from(x))
            .collect::<Vec<String>>();
        let it = result_vec_2.iter().zip(result_vec.iter());
        let map: HashMap<&String, &String> = it.collect();
        let map2: HashMap<String, String> = map
            .clone()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        map2
    }

    match year {
        d if d <= 1699 => tt(4),
        d if d <= 1799 => tt(3),
        d if d <= 1899 => tt(2),
        d if d <= 2099 => tt(1),
        _ => panic!(),
    }
}

pub trait GoldenNumber {
    fn get_golden_number(&self) -> u8;
}

pub trait Concurrents {
    fn get_concurrents(&self) -> u8;
}

pub trait SolarCircle {
    fn get_solar_circle(&self) -> u8;
}

pub trait DominicalLetter {
    fn get_dominical_letter_j(&self) -> String;
    fn get_dominical_letter_g(&self) -> String;
}

pub trait Epact {
    fn get_epact_j(&self) -> u8;
    fn get_epact_g(&self) -> u8;
}

pub trait Eastern {
    fn eastern_j(&self) -> NaiveDate;
    fn eastern_g(&self) -> NaiveDate;
}

pub struct Chrono {
    year: i16,
    gregorian: bool,
}

impl Chrono {
    pub fn new(year: i16, gregorian: bool) -> Chrono {
        Chrono { year, gregorian }
    }
}

impl Eastern for Chrono {
    fn eastern_j(&self) -> NaiveDate {
        let help_a: u16 = (self.year % 19) as u16;
        let help_b: u16 = (self.year % 4) as u16;
        let help_c: u16 = (self.year % 7) as u16;
        let help_d: u16 = ((15 + (19 * help_a)) % 30) as u16;
        let help_e: u16 = ((6 + (2 * help_b) + (4 * help_c) + (6 * help_d)) % 7) as u16;

        if (22 + help_d + help_e) > 31 {
            let date_fomat: String = format!("{}-{}-{}", self.year, 4, (help_d + help_e - 9));
            let date_only = NaiveDate::parse_from_str(&date_fomat, "%Y-%m-%d");
            match date_only {
                Ok(n) => n,
                Err(_err) => panic!(),
            }
        } else {
            let date_fomat: String = format!("{}-{}-{}", self.year, 3, (22 + help_d + help_e));
            let date_only = NaiveDate::parse_from_str(&date_fomat, "%Y-%m-%d");
            match date_only {
                Ok(n) => n,
                Err(_err) => panic!(),
            }
        }
    }

    fn eastern_g(&self) -> NaiveDate {
        NaiveDate::from_num_days_from_ce(20)
    }
}

impl GoldenNumber for Chrono {
    fn get_golden_number(&self) -> u8 {
        let golden_number: u8 = ((self.year + 1) % 19) as u8;
        if golden_number == 0 {
            return 19;
        }
        golden_number
    }
}

impl Concurrents for Chrono {
    fn get_concurrents(&self) -> u8 {
        let quarter = self.year / 4;
        let concurrents: u8 = ((self.year + quarter + 4) % 7) as u8;
        if concurrents == 0 {
            return 7;
        }
        concurrents
    }
}

impl SolarCircle for Chrono {
    fn get_solar_circle(&self) -> u8 {
        let solar_mod: u8 = ((self.year + 9) % 28) as u8;
        if solar_mod == 0 {
            return 28;
        }
        solar_mod
    }
}

impl Epact for Chrono {
    fn get_epact_j(&self) -> u8 {
        let golden_number = &self::GoldenNumber::get_golden_number(self);
        ((golden_number - 1) * 11) % 30
    }

    fn get_epact_g(&self) -> u8 {
        if self.gregorian {
            let century: u8 = ((self.year / 100) + 1) as u8;
            let correction_solar: u8 = (3 * century / 4) as u8;
            let correction_lunar: u8 = ((8.0 * f32::from(century) + 5.0) / 25.0).trunc() as u8;
            let epact_j: u8 = self::Epact::get_epact_j(self);
            ((epact_j as i8 - correction_solar as i8 + correction_lunar as i8 + 8) % 30) as u8
        } else {
            panic!()
        }
    }
}

impl DominicalLetter for Chrono {
    fn get_dominical_letter_j(&self) -> String {
        let solar_circle = &self::SolarCircle::get_solar_circle(self);
        match generate_d_map().get(solar_circle) {
            Some(letter) => letter.clone(),
            None => panic!(),
        }
    }

    fn get_dominical_letter_g(&self) -> String {
        match generate_d_g_map(self.year).get(&self.get_dominical_letter_j()) {
            Some(letter) => letter.clone(),
            None => panic!(),
        }
    }
}