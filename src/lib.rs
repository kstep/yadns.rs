#![feature(custom_derive, plugin)]
#![feature(custom_attribute)]

extern crate hyper;
extern crate url;
extern crate serde;
extern crate serde_json;

mod skiperr;
pub mod dto;
pub mod error;
pub mod api;

use std::borrow::{Cow, Borrow};
use std::convert::AsRef;
use std::error::Error as StdError;
use std::fmt;
use std::io::{Read, Error as IoError};
use std::net::{Ipv4Addr, Ipv6Addr};
use std::ops::{Deref, DerefMut};
use std::result::Result as StdResult;

use url::form_urlencoded;
use hyper::header::{Header, HeaderFormat, ContentType};
use hyper::method::Method;
use hyper::{Client, Error as HttpError};

