extern crate chrono;

use chrono::NaiveDate;
use std::collections::HashMap;
use chrono::Duration;

const SIMPLE_ARRAY: [&str; 7] = ["A", "B", "C", "D", "E", "F", "G"];
const DOUBLE_ARRAY: [&str; 7] = ["GF", "BA", "DC", "FE", "AG", "CB", "ED"];

fn generate_d_map() -> HashMap<u8, &'static str> {
    let dom_let_array = [
        "GF", "E", "D", "C", "BA", "G", "F", "E", "DC", "B", "A", "G", "FE", "D", "C", "B", "AG",
        "F", "E", "D", "CB", "A", "G", "F", "ED", "C", "B", "A",
    ];

    dom_let_array
        .iter()
        .enumerate()
        .map(|(k, &v)| ((k + 1) as u8, v))
        .collect()
}

fn generate_d_g_map(year: i16) -> HashMap<&'static str, &'static str> {
    fn generate_map(shift_count: usize) -> HashMap<&'static str, &'static str> {
        let mut copy_simple = SIMPLE_ARRAY;
        let mut copy_double = DOUBLE_ARRAY;
        copy_simple.rotate_right(shift_count);
        copy_double.rotate_right(shift_count);

        let shifted = [copy_simple, copy_double]
            .concat();

        [SIMPLE_ARRAY, DOUBLE_ARRAY].concat()
            .iter()
            .zip(shifted)
            .map(|(&k, v)| (k, v))
            .collect()
    }

    match year {
        d if d <= 1699 => generate_map(4),
        d if d <= 1799 => generate_map(3),
        d if d <= 1899 => generate_map(2),
        d if d <= 2099 => generate_map(1),
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
    fn get_dominical_letter_j(&self) -> &str;
    fn get_dominical_letter_g(&self) -> &str;
}

pub trait Epact {
    fn get_epact_j(&self) -> u8;
    fn get_epact_g(&self) -> u8;
}

pub trait Eastern {
    fn eastern_j(&self) -> NaiveDate;
    fn eastern_g(&self) -> NaiveDate;
}

pub trait MovableFeast {
    fn add(&self, date: NaiveDate, quantity: i64) -> NaiveDate;
    fn deduct(&self, date: NaiveDate, quantity: i64) -> NaiveDate;
    fn septuagesima(&self) -> NaiveDate;
    fn quinquagesina(&self) -> NaiveDate;
    fn sexagesima(&self) -> NaiveDate;
    fn dies_cinerum(&self) -> NaiveDate;
    fn reminiscere(&self) -> NaiveDate;
    fn second_sun_lent(&self) -> NaiveDate;
    fn kychava_nedele(&self) -> NaiveDate;
    fn druzebna_nedele(&self) -> NaiveDate;
    fn smrtna_nedele(&self) -> NaiveDate;
    fn kvetna_nedele(&self) -> NaiveDate;
    fn zeleny_ctvrtek(&self) -> NaiveDate;
    fn velky_patek(&self) -> NaiveDate;
    fn bila_sobota(&self) -> NaiveDate;
    fn bila_nedele(&self) -> NaiveDate;
    fn den_svatosti(&self) -> NaiveDate;
    fn misericordia(&self) -> NaiveDate;
    fn jubilate(&self) -> NaiveDate;
    fn exaudi(&self) -> NaiveDate;
    fn letnice(&self) -> NaiveDate;
    fn trojice(&self) -> NaiveDate;
    fn boziho_tela(&self) -> NaiveDate;
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

macro_rules! movable_macro {
    ($name:ident, $years:expr, $op:ident, $days:expr, $daysExp:expr) => {
        fn $name(&self) -> NaiveDate {
        match self.year {
            1300 | 1400 | 1500 => self.$op(
                if self.gregorian { self.eastern_g() } else { self.eastern_j() },
                $days,
            ),
            _ => self.$op(
                if self.gregorian { self.eastern_g() } else { self.eastern_j() },
                $daysExp,
            )
        }
    }
    };
    ($name:ident, $op:ident, $days:expr) => {
        fn $name(&self) -> NaiveDate {
        self.$op(
                if self.gregorian { self.eastern_g() } else { self.eastern_j() },
                $days,
            )
    }
}
}

impl MovableFeast for Chrono {
    fn add(&self, date: NaiveDate, quantity: i64) -> NaiveDate {
        date + Duration::days(quantity)
    }

    fn deduct(&self, date: NaiveDate, quantity: i64) -> NaiveDate {
        date - Duration::days(quantity)
    }

    movable_macro!(septuagesima, true, deduct, 62, 7 * 9);
    movable_macro!(quinquagesina, true, deduct, 48, 7 * 7);
    movable_macro!(sexagesima, true, deduct, 55, 7 * 8);
    movable_macro!(reminiscere, true, deduct, 41, 7 * 6);
    movable_macro!(dies_cinerum, true, deduct, 45, 46);
    movable_macro!(second_sun_lent, deduct, 7 * 5);
    movable_macro!(kychava_nedele, deduct, 7 * 4);
    movable_macro!(druzebna_nedele, deduct, 7 * 3);
    movable_macro!(smrtna_nedele, deduct, 7 * 2);
    movable_macro!(kvetna_nedele, deduct, 7);
    movable_macro!(zeleny_ctvrtek, deduct, 3);
    movable_macro!(velky_patek, deduct, 2);
    movable_macro!(bila_sobota, deduct, 1);
    movable_macro!(bila_nedele, add, 7);
    movable_macro!(den_svatosti, add, 12);
    movable_macro!(misericordia, add, 7 * 2);
    movable_macro!(jubilate, add, 7 * 3);
    movable_macro!(exaudi, add, 7 * 6);
    movable_macro!(letnice, add, 7 * 7);
    movable_macro!(trojice, add, 7 * 8);
    movable_macro!(boziho_tela, add, 60);
}

impl Eastern for Chrono {
    fn eastern_j(&self) -> NaiveDate {
        let help_a: u16 = (self.year % 19) as u16;
        let help_b: u16 = (self.year % 4) as u16;
        let help_c: u16 = (self.year % 7) as u16;
        let help_d: u16 = ((15 + (19 * help_a)) % 30) as u16;
        let help_e: u16 = ((6 + (2 * help_b) + (4 * help_c) + (6 * help_d)) % 7) as u16;

        if (22 + help_d + help_e) > 31 {
            let date_format: String = format!("{}-{}-{}", self.year, 4, (help_d + help_e - 9));
            NaiveDate::parse_from_str(&date_format, "%Y-%m-%d")
                .unwrap()
        } else {
            let date_format: String = format!("{}-{}-{}", self.year, 3, (22 + help_d + help_e));
            NaiveDate::parse_from_str(&date_format, "%Y-%m-%d")
                .unwrap()
        }
    }

    fn eastern_g(&self) -> NaiveDate {
        if self.gregorian {
            let help_a: u16 = (self.year % 19) as u16;
            let help_b: u16 = (self.year >> 2) as u16;
            let help_c: u16 = (help_b / 25 + 1) as u16;
            let mut help_d: u16 = ((help_c * 3) >> 2) as u16;
            let mut help_e: u16 = (((help_a * 19) - ((help_c * 8 + 5) / 25) + help_d + 15) % 30) as u16;
            help_e += (29_578 - help_a - help_e * 32) >> 10;
            help_e -= ((self.year as u16 % 7) + help_b - help_d + help_e + 2) % 7;
            help_d = help_e >> 5;
            let day: u8 = (help_e - help_d * 31) as u8;
            let month: u8 = (help_d + 3) as u8;
            let date_format: String = format!("{}-{}-{}", self.year, month, day);
            NaiveDate::parse_from_str(&date_format, "%Y-%m-%d")
                .unwrap()
        } else {
            panic!()
        }
    }
}

impl GoldenNumber for Chrono {
    fn get_golden_number(&self) -> u8 {
        match (self.year + 1) % 19 {
            0 => 19,
            n => n as u8,
        }
    }
}

impl Concurrents for Chrono {
    fn get_concurrents(&self) -> u8 {
        let quarter = self.year / 4;
        match (self.year + quarter + 4) % 7 {
            0 => 7,
            n => n as u8,
        }
    }
}

impl SolarCircle for Chrono {
    fn get_solar_circle(&self) -> u8 {
        match (self.year + 9) % 28 {
            0 => 28,
            n => n as u8,
        }
    }
}

impl Epact for Chrono {
    fn get_epact_j(&self) -> u8 {
        ((self.get_golden_number() - 1) * 11) % 30
    }

    fn get_epact_g(&self) -> u8 {
        if self.gregorian {
            let century: u8 = ((self.year / 100) + 1) as u8;
            let correction_solar: u8 = (3 * century / 4) as u8;
            let correction_lunar: u8 = ((8.0 * f32::from(century) + 5.0) / 25.0).trunc() as u8;
            let epact_j: u8 = self.get_epact_j();
            ((epact_j as i8 - correction_solar as i8 + correction_lunar as i8 + 8) % 30) as u8
        } else {
            panic!()
        }
    }
}

impl DominicalLetter for Chrono {
    fn get_dominical_letter_j(&self) -> &str {
        generate_d_map()
            .get(&self.get_solar_circle())
            .unwrap()
    }

    fn get_dominical_letter_g(&self) -> &str {
        generate_d_g_map(self.year)
            .get(&self.get_dominical_letter_j())
            .unwrap()
    }
}
