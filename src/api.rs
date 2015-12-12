
use super::dto::ErrorReply;
use super::error::Result;

use std::result::Result as StdResult;
use std::fmt;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::io::Read;

use serde_json;
use hyper::Client;
use hyper::error::Error as HttpError;
use hyper::method::Method;
use hyper::header::{Header, HeaderFormat, ContentType};
use serde::{de, Deserialize, Deserializer};
use url::form_urlencoded;

static BASE_URL: &'static str = "https://pddimp.yandex.ru/api2/admin/dns";

pub struct YandexDNS {
    token: PddToken,
    client: Client
}

impl YandexDNS {
    pub fn new(token: &str) -> YandexDNS {
        YandexDNS {
            token: PddToken(token.to_owned()),
            client: Client::new()
        }
    }

    pub fn call<R: Deserialize>(&mut self, func: &str, method: Method, args: &[(&str, &str)]) -> Result<R> {
        let url;
        let params = form_urlencoded::serialize(args);

        let mut resp = try! {
            match method {
                Method::Get | Method::Delete => {
                    url = format!("{}/{}?{}", BASE_URL, func, params);
                    self.client.request(method, &*url)
                },
                _ => {
                    url = format!("{}/{}", BASE_URL, func);
                    self.client.request(method, &*url).body(&*params)
                },
            }
            .header(self.token.clone())
            .header(ContentType("application/x-www-form-urlencoded".parse().unwrap()))
            .send()
        };

        let data = {
            let mut buf = String::new();
            try!(resp.read_to_string(&mut buf));
            buf
        };

        //println!("{}", data);

        try!(serde_json::from_str::<R>(&*data).map(Ok).or_else(
                |e| serde_json::from_str::<ErrorReply>(&*data).map(Err).map_err(|_| e))
        ).map_err(From::from)
    }

    pub fn send<V: YaVisitor>(&mut self, visitor: &V) -> Result<V::Reply> {
        visitor.visit(self)
    }

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PddToken(String);

impl Header for PddToken {
    fn header_name() -> &'static str {
        "PddToken"
    }

    fn parse_header(raw: &[Vec<u8>]) -> StdResult<PddToken, HttpError> {
        Ok(PddToken(String::from_utf8_lossy(&*raw[0]).into_owned()))
    }
}

impl HeaderFormat for PddToken {
    fn fmt_header(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let PddToken(ref value) = *self;
        fmt.write_str(&**value)
    }
}

#[derive(Debug)]
pub enum Content {
    Ipv4(Ipv4Addr),
    Ipv6(Ipv6Addr),
    Info(String),
}

impl Deserialize for Content {
    fn deserialize<D: Deserializer>(d: &mut D) -> StdResult<Content, D::Error> {
        use self::Content::*;
        let info: String = try!(Deserialize::deserialize(d));
        Ok(info.parse::<Ipv4Addr>().map(Ipv4)
        .or_else(|_| info.parse::<Ipv6Addr>().map(Ipv6))
        .unwrap_or_else(|_| Info(info)))
    }
}

pub trait YaVisitor {
    type Reply: Deserialize;
    fn visit(&self, api: &mut YandexDNS) -> Result<Self::Reply>;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DnsType {
    Srv,
    Txt,
    Ns,
    Mx,
    Soa,
    A,
    Aaaa,
    Cname,
}

impl AsRef<str> for DnsType {
    fn as_ref(&self) -> &str {
        use self::DnsType::*;
        match *self {
            Srv => "SRV",
            Txt => "TXT",
            Ns => "NS",
            Mx => "MX",
            Soa => "SOA",
            A => "A",
            Aaaa => "AAAA",
            Cname => "CNAME",
        }
    }
}

impl Deserialize for DnsType {
    fn deserialize<D: Deserializer>(d: &mut D) -> StdResult<DnsType, D::Error> {
        struct DnsTypeVisitor;

        impl de::Visitor for DnsTypeVisitor {
            type Value = DnsType;
            fn visit_str<E: de::Error>(&mut self, v: &str) -> StdResult<DnsType, E> {
                use self::DnsType::*;
                match v {
                    "SRV" => Ok(Srv),
                    "TXT" => Ok(Txt),
                    "NS" => Ok(Ns),
                    "MX" => Ok(Mx),
                    "SOA" => Ok(Soa),
                    "A" => Ok(A),
                    "AAAA" => Ok(Aaaa),
                    "CNAME" => Ok(Cname),
                    _ => Err(de::Error::unknown_field("unknown record type"))
                }
            }
        }

        d.visit(DnsTypeVisitor)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ResultCode {
    Ok,
    Err,
}

impl Deserialize for ResultCode {
    fn deserialize<D: Deserializer>(d: &mut D) -> StdResult<ResultCode, D::Error> {
        struct ResultCodeVisitor;

        impl de::Visitor for ResultCodeVisitor {
            type Value = ResultCode;
            fn visit_str<E: de::Error>(&mut self, v: &str) -> StdResult<ResultCode, E> {
                match v {
                    "ok" => Ok(ResultCode::Ok),
                    "error" => Ok(ResultCode::Err),
                    _ => Err(de::Error::unknown_field("invalid result code"))
                }
            }
        }

        d.visit_str(ResultCodeVisitor)
    }
}

