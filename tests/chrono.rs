use chrono::NaiveDate;
use historical_chrono::{Concurrents, MovableFeast};
use historical_chrono::Epact;
use historical_chrono::GoldenNumber;
use historical_chrono::SolarCircle;
use historical_chrono::{Chrono, Eastern};
use historical_chrono::DominicalLetter;

#[test]
fn test_dominical_letter_g() {
  assert_eq!("D", Chrono::new(2015, true).get_dominical_letter_g());
  assert!(std::panic::catch_unwind(|| Chrono::new(3000, true).get_dominical_letter_g().to_string()).is_err());
}

#[test]
fn test_dominical_letter_j() {
  assert_eq!("E", Chrono::new(2015, false).get_dominical_letter_j());
}

#[test]
fn test_solar_circle() {
    assert_eq!(14, Chrono::new(341, true).get_solar_circle());
    assert_eq!(15, Chrono::new(1126, true).get_solar_circle());
    assert_eq!(24, Chrono::new(1583, true).get_solar_circle());
    assert_eq!(8, Chrono::new(2015, true).get_solar_circle());
}

#[test]
fn test_concurrents() {
    assert_eq!(3, Chrono::new(341, true).get_concurrents());
    assert_eq!(4, Chrono::new(1126, true).get_concurrents());
    assert_eq!(1, Chrono::new(1583, true).get_concurrents());
    assert_eq!(2, Chrono::new(2015, true).get_concurrents());
}

#[test]
fn test_golden_number() {
    assert_eq!(19, Chrono::new(341, true).get_golden_number());
    assert_eq!(6, Chrono::new(1126, true).get_golden_number());
    assert_eq!(7, Chrono::new(1583, true).get_golden_number());
    assert_eq!(2, Chrono::new(2015, true).get_golden_number());
}

#[test]
fn test_epact_g() {
    assert!(std::panic::catch_unwind(|| Chrono::new(341, false).get_epact_g()).is_err());
    assert!(std::panic::catch_unwind(|| Chrono::new(1126, false).get_epact_g()).is_err());
    assert!(std::panic::catch_unwind(|| Chrono::new(1583, false).get_epact_g()).is_err());
    assert_eq!(10, Chrono::new(2015, true).get_epact_g());
}

#[test]
fn test_epact_j() {
    assert_eq!(18, Chrono::new(341, true).get_epact_j());
    assert_eq!(25, Chrono::new(1126, true).get_epact_j());
    assert_eq!(6, Chrono::new(1583, true).get_epact_j());
    assert_eq!(11, Chrono::new(2015, true).get_epact_j());
}

#[test]
fn test_eastern_j() {
    assert_eq!(
        NaiveDate::parse_from_str("1126-4-11", "%Y-%m-%d").unwrap(),
        Chrono::new(1126, true).eastern_j()
    );
    assert_eq!(
        NaiveDate::parse_from_str("1573-3-22", "%Y-%m-%d").unwrap(),
        Chrono::new(1573, true).eastern_j()
    );
    assert_eq!(
        NaiveDate::parse_from_str("1410-3-23", "%Y-%m-%d").unwrap(),
        Chrono::new(1410, true).eastern_j()
    );
    assert_eq!(
        NaiveDate::parse_from_str("1692-3-27", "%Y-%m-%d").unwrap(),
        Chrono::new(1692, true).eastern_j()
    );
    assert_eq!(
        NaiveDate::parse_from_str("1492-4-22", "%Y-%m-%d").unwrap(),
        Chrono::new(1492, true).eastern_j()
    );
}

#[test]
fn test_eastern_g() {
    assert_eq!(
        NaiveDate::parse_from_str("2015-4-5", "%Y-%m-%d").unwrap(),
        Chrono::new(2015, true).eastern_g()
    );
    assert!(std::panic::catch_unwind(|| Chrono::new(2015, false)
        .eastern_g())
        .is_err());
}

#[test]
fn test_septuagesima() {
    assert_eq!(
        NaiveDate::parse_from_str("1126-2-7", "%Y-%m-%d").unwrap(),
        Chrono::new(1126, false).septuagesima()
    );
    assert_eq!(
        NaiveDate::parse_from_str("1666-2-21", "%Y-%m-%d").unwrap(),
        Chrono::new(1666, true).septuagesima()
    );
    assert_eq!(
        NaiveDate::parse_from_str("1791-2-20", "%Y-%m-%d").unwrap(),
        Chrono::new(1791, true).septuagesima()
    );
    assert_eq!(
        NaiveDate::parse_from_str("1272-2-21", "%Y-%m-%d").unwrap(),
        Chrono::new(1272, false).septuagesima()
    );
    assert_eq!(
        NaiveDate::parse_from_str("1522-2-16", "%Y-%m-%d").unwrap(),
        Chrono::new(1522, false).septuagesima()
    );
    assert_eq!(
        NaiveDate::parse_from_str("1764-2-19", "%Y-%m-%d").unwrap(),
        Chrono::new(1764, true).septuagesima()
    );
    assert_eq!(
        NaiveDate::parse_from_str("1300-2-7", "%Y-%m-%d").unwrap(),
        Chrono::new(1300, false).septuagesima()
    );
}

#[test]
fn test_misericordia() {
    assert_eq!(
        NaiveDate::parse_from_str("1126-4-25", "%Y-%m-%d").unwrap(),
        Chrono::new(1126, false).misericordia()
    );
    assert_eq!(
        NaiveDate::parse_from_str("1443-5-5", "%Y-%m-%d").unwrap(),
        Chrono::new(1443, false).misericordia()
    );
    assert_eq!(
        NaiveDate::parse_from_str("1737-5-5", "%Y-%m-%d").unwrap(),
        Chrono::new(1737, true).misericordia()
    );
    assert_eq!(
        NaiveDate::parse_from_str("1598-4-5", "%Y-%m-%d").unwrap(),
        Chrono::new(1598, true).misericordia()
    );
    assert_eq!(
        NaiveDate::parse_from_str("1690-4-9", "%Y-%m-%d").unwrap(),
        Chrono::new(1690, true).misericordia()
    );
}
