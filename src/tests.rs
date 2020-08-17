use crate::types::CiString;
use serde_json::json;

const SEED: &str = "CaFeBaBe";
const UPPER: &str = "CAFEBABE";
const LOWER: &str = "cafebabe";
const INVERTED: &str = "cAfEbAbE";

fn test_eq(sut: CiString) {
    assert_eq!(sut, sut);

    assert_eq!(sut, SEED);
    assert_eq!(sut, UPPER);
    assert_eq!(sut, LOWER);
    assert_eq!(sut, INVERTED);

    assert_eq!(sut, String::from(SEED));
    assert_eq!(sut, String::from(UPPER));
    assert_eq!(sut, String::from(LOWER));
    assert_eq!(sut, String::from(INVERTED));

    assert_eq!(sut, CiString::from(SEED));
    assert_eq!(sut, CiString::from(UPPER));
    assert_eq!(sut, CiString::from(LOWER));
    assert_eq!(sut, CiString::from(INVERTED));

    let s: String = sut.into();
    assert_eq!(s, String::from(SEED));
}

#[test]
fn test_from_str() {
    let sut = CiString::from(SEED);
    test_eq(sut);
}

#[test]
fn test_from_string() {
    let sut = CiString::from(SEED.to_string());
    test_eq(sut);
}

#[test]
fn test_deserialization() {
    let sut = serde_json::from_value::<CiString>(json!(SEED)).unwrap();
    test_eq(sut);
}

#[test]
fn test_roundtrip() {
    let original = String::from(SEED);
    let ci = CiString::from(SEED);
    let roundtripped: String = ci.into();

    assert_eq!(original, roundtripped);
}

#[test]
fn test_format_keeps_capitalization() {
    let fmt = format!("{}", CiString::from(SEED));
    assert_eq!(fmt, SEED);
}