#![cfg_attr(feature = "nightly", feature(custom_derive, custom_attribute, plugin))]
#![cfg_attr(feature = "nightly", plugin(serde_macros))]

extern crate hyper;
extern crate url;
extern crate serde;
extern crate serde_json;

mod skiperr;
pub mod dto;
pub mod error;
pub mod api;

pub use api::{YandexDNS, DnsType};
pub use dto::{ListRequest, AddRequest, EditRequest, DeleteRequest};
pub use dto::{ListReply, AddReply, EditReply, DeleteReply};
