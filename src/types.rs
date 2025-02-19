use crate::sql_types::*;
use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Text;
use diesel::backend::Backend;
use schemars::{JsonSchema};
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::cmp::{Eq, PartialEq};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::str::FromStr;

#[cfg(feature = "with-actix-web")]
use actix_web::dev::FromParam;

/// `CiString` is a CaseInsensitive String type that can be used as the key for
/// a hashmap as well as be written to the page. It implements a variety of traits
/// to make it easy to convert from and to &str and String types.
#[derive(Clone, Debug, Default, PartialOrd, Ord, Eq)]
#[derive(Deserialize, Serialize)]
#[derive(AsExpression, FromSqlRow)]
#[derive(JsonSchema)]
#[serde(transparent)]
#[diesel(sql_type = Citext)]
pub struct CiString(String);

impl CiString {
    pub fn new() -> Self {
        Self::default()
    }
}

/// CiString can implement the FromParam trait from ActixWeb if "with-actix-web" is
/// turned on in the Cargo.toml file.
#[cfg(feature = "with-actix-web")]
impl FromParam for CiString {
    type Err = actix_web::error::UrlParseError;

    fn from_param(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl fmt::Display for CiString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq for CiString {
    fn eq(&self, other: &CiString) -> bool {
        self.0.to_lowercase() == other.0.to_lowercase()
    }
}

impl PartialEq<String> for CiString {
    fn eq(&self, other: &String) -> bool {
        self.0.to_lowercase() == other.to_lowercase()
    }
}

impl PartialEq<&str> for CiString {
    fn eq(&self, other: &&str) -> bool {
        self.0.to_lowercase() == other.to_lowercase()
    }
}

impl Hash for CiString {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.0.to_lowercase().hash(hasher);
    }
}

impl AsRef<str> for CiString {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Borrow<str> for CiString {
    fn borrow(&self) -> &str {
        self.0.borrow()
    }
}

impl Deref for CiString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for CiString {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl From<CiString> for String {
    fn from(value: CiString) -> Self {
        value.0
    }
}

impl From<String> for CiString {
    fn from(value: String) ->  Self {
        CiString(value)
    }
}

impl From<&str> for CiString {
    fn from(value: &str) ->  Self {
        CiString(value.into())
    }
}


impl<DB> FromSql<Citext, DB> for CiString
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: diesel::backend::RawValue<'_, DB>) -> deserialize::Result<Self> {
        String::from_sql(bytes)
            .map(Self::from)
    }
}

impl<DB> ToSql<Citext, DB> for CiString
where
    DB: Backend,
    String: ToSql<Text, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
        self.0.to_sql(out)
    }
}

impl FromSql<Citext, Pg> for String {
    fn from_sql(bytes: diesel::backend::RawValue<'_, Pg>) -> deserialize::Result<Self> {
        FromSql::<Text, Pg>::from_sql(bytes)
    }
}

impl<DB> ToSql<Citext, DB> for String
where
    DB: Backend,
    String: ToSql<Text, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
        ToSql::<Text, DB>::to_sql(self, out)
    }
}

impl<DB> ToSql<Citext, DB> for str
where
    DB: Backend,
    str: ToSql<Text, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
        ToSql::<Text, DB>::to_sql(self, out)
    }
}
