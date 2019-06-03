//! Diesel support for Postgres citext Extension

#![allow(proc_macro_derive_resolution_fallback)]
#[macro_use]
extern crate diesel;

extern crate serde;

#[cfg(feature = "with-actix-web")]
extern crate actix_web;

#[cfg(test)]
extern crate serde_json;

pub mod sql_types;
pub mod types;

#[cfg(test)]
mod tests;
