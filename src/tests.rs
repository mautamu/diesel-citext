use crate::types::CiString;
use serde_json::json;

const SEED: &str = "CaFeBaBe";
const UPPER: &str = "CAFEBABE";
const LOWER: &str = "cafebabe";
const INVERTED: &str = "cAfEbAbE";

#[test]
fn it_works() {
    let v = json!(SEED);
    let s: CiString = serde_json::from_value(v).unwrap();

    assert_eq!(s, SEED);
    assert_eq!(s, UPPER);
    assert_eq!(s, LOWER);
    assert_eq!(s, INVERTED);
}
