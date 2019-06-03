# CHANGE Log

## 0.4.0

* Serialize and Deserialize CiString as plain type
* FromStr for CiString now returns Result<CiString, ()> instead of Result<CiString, fmt::Error>

## 0.3.0

* Add Partial Eq Support
* Set SQL Types Macro to fix error message for Insertable Derives

## 0.2.0

* Add CiString Type and supporting methods
