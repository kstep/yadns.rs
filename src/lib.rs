#![feature(custom_derive, plugin)]
#![feature(custom_attribute)]
#![plugin(serde_macros)]

extern crate hyper;
extern crate url;
extern crate serde;
extern crate serde_json;

mod skiperr;
pub mod dto;
pub mod error;
pub mod api;

