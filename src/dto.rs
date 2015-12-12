use super::api::{DnsType, Content, ResultCode, YandexDNS, YaVisitor};
use super::error::{ErrorCode, Result};
use super::skiperr::SkipErr;

use std::borrow::{Cow, Borrow};

use hyper::method::Method;

macro_rules! as_str {
    ($key:ident) => { stringify!($key) };
    ($key:expr) => { $key };
}
macro_rules! qs {
    ($($key:tt => $value:expr),* $(,)*) => {
        &[$((as_str!($key), $value)),*]
    }
}
macro_rules! opt_borrow {
    ($val:expr) => {
        match $val { None => "", Some(ref val) => &**val }
    };
    (str $val:expr) => {
        match $val { None => "", Some(ref val) => &*val.to_string() }
    };
}

#[derive(Debug, Deserialize)]
pub struct Record {
    record_id: u64,
    #[serde(rename="type")]
    kind: DnsType,
    domain: String,
    subdomain: String,
    fqdn: String,
    content: Content,
    ttl: u32,

    priority: SkipErr<u32>,

    // SOA
    refresh: Option<u32>,
    admin_mail: Option<String>,
    expire: Option<u32>,
    minttl: Option<u32>,
    retry: Option<u32>,

    // SRV
    weight: Option<u32>,
    port: Option<u16>,

    // edit
    operation: Option<String>,
}

impl Record {
    pub fn as_add_req(&self) -> AddRequest {
        AddRequest {
            domain: (&*self.domain).into(),
            kind: self.kind,

            admin_mail: self.admin_mail.as_ref().map(|v| (&**v).into()).unwrap_or("".into()),
            content: match self.content {
                Content::Ipv4(ref ip) => ip.to_string().into(),
                Content::Ipv6(ref ip) => ip.to_string().into(),
                Content::Info(ref info) => (&**info).into(),
            },
            priority: self.priority.unwrap_or(10),
            weight: self.weight.unwrap_or(0),
            port: self.port.unwrap_or(0),
            target: "".into(),

            subdomain: (&*self.subdomain).into(),
            ttl: self.ttl,
        }
    }
    pub fn as_edit_req(&self) -> EditRequest {
        EditRequest {
            domain: (&*self.domain).into(),
            record_id: self.record_id,

            subdomain: Some((&*self.subdomain).into()),
            ttl: Some(self.ttl),
            refresh: self.refresh,
            retry: self.retry,
            expire: self.expire,
            neg_cache: None,
            admin_mail: self.admin_mail.as_ref().map(|v| (&**v).into()),
            content: Some(match self.content {
                Content::Ipv4(ref ip) => ip.to_string().into(),
                Content::Ipv6(ref ip) => ip.to_string().into(),
                Content::Info(ref info) => (&**info).into(),
            }),
            priority: self.priority.clone(),
            port: self.port,
            weight: self.weight,
            target: None,
        }
    }
    pub fn as_delete_req(&self) -> DeleteRequest {
        DeleteRequest {
            domain: (&*self.domain).into(),
            record_id: self.record_id,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ListReply {
    pub records: Vec<Record>,
    pub domain: String,
    pub success: ResultCode,
}

#[derive(Debug, Deserialize)]
pub struct EditReply {
    pub domain: String,
    pub record_id: u64,
    pub record: Record,
    pub success: ResultCode,
}

#[derive(Debug, Deserialize)]
pub struct AddReply {
    pub domain: String,
    pub record: Record,
    pub success: ResultCode,
}

#[derive(Debug, Deserialize)]
pub struct DeleteReply {
    pub domain: String,
    pub record_id: u64,
    pub success: ResultCode,
}

#[derive(Debug, Deserialize)]
pub struct ErrorReply {
    pub domain: String,
    pub record_id: Option<u64>,
    pub success: ResultCode,
    pub error: ErrorCode,
}

#[derive(Debug)]
pub struct ListRequest<'a> {
    domain: Cow<'a, str>,
}

#[derive(Debug)]
pub struct AddRequest<'a> {
    domain: Cow<'a, str>,
    kind: DnsType,

    admin_mail: Cow<'a, str>, // required for SOA
    content: Cow<'a, str>, // Ipv4 for A, Ipv6 for AAAA, string for CNAME, MX, NS, TXT
    priority: u32, // required for SRV and MX, default: 10
    weight: u32, // required for SRV
    port: u16, // required for SRV
    target: Cow<'a, str>, // required for SRV

    subdomain: Cow<'a, str>,
    ttl: u32, // default: 21600
}

#[derive(Debug)]
pub struct EditRequest<'a> {
    domain: Cow<'a, str>,
    record_id: u64,

    subdomain: Option<Cow<'a, str>>, // default: "@"
    ttl: Option<u32>, // default: 21600, 900...21600
    refresh: Option<u32>, // for SOA, default: 10800, 900...86400
    retry: Option<u32>, // for SOA, default: 900, 90...3600
    expire: Option<u32>, // for SOA, default: 900, 90...3600
    neg_cache: Option<u32>, // for SOA, default: 10800, 90...86400
    admin_mail: Option<Cow<'a, str>>, // required for SOA
    content: Option<Cow<'a, str>>, // Ipv4 for A, Ipv6 for AAAA, string for CNAME, MX, NS, TXT
    priority: Option<u32>, // required for SRV and MX, default: 10
    port: Option<u16>, // required for SRV
    weight: Option<u32>, // required for SRV
    target: Option<Cow<'a, str>>, // required for SRV
}

#[derive(Debug)]
pub struct DeleteRequest<'a> {
    domain: Cow<'a, str>,
    record_id: u64,
}

impl<'a> ListRequest<'a> {
    pub fn new<T: Into<Cow<'a, str>>>(domain: T) -> ListRequest<'a> {
        ListRequest {
            domain: domain.into(),
        }
    }
}

impl<'a> AddRequest<'a> {
    pub fn new<T: Into<Cow<'a, str>>>(kind: DnsType, domain: T) -> AddRequest<'a> {
        AddRequest {
            domain: domain.into(),
            kind: kind,

            admin_mail: "".into(),
            content: "".into(),
            priority: 10,
            weight: 0,
            port: 0,
            target: "".into(),
            subdomain: "@".into(),
            ttl: 21600,
        }
    }

    pub fn subdomain<T: Into<Cow<'a, str>>>(&mut self, value: T) -> &mut Self {
        self.subdomain = value.into();
        self
    }

    pub fn content<T: Into<Cow<'a, str>>>(&mut self, value: T) -> &mut Self {
        self.content = value.into();
        self
    }
}

impl<'a> EditRequest<'a> {
    pub fn subdomain<T: Into<Cow<'a, str>>>(&mut self, value: T) -> &mut Self {
        self.subdomain = Some(value.into());
        self
    }

    pub fn content<T: Into<Cow<'a, str>>>(&mut self, value: T) -> &mut Self {
        self.content = Some(value.into());
        self
    }
}

impl<'a> YaVisitor for ListRequest<'a> {
    type Reply = ListReply;
    fn visit(&self, api: &mut YandexDNS) -> Result<Self::Reply> {
        api.call("list", Method::Get, qs! {
            domain => self.domain.borrow(),
        })
    }
}

impl<'a> YaVisitor for AddRequest<'a> {
    type Reply = AddReply;
    fn visit(&self, api: &mut YandexDNS) -> Result<Self::Reply> {
        api.call("add", Method::Post, qs! {
            domain => self.domain.borrow(),
            type => self.kind.as_ref(),

            admin_mail => self.admin_mail.borrow(),
            content => self.content.borrow(),
            priority => &*self.priority.to_string(),
            weight => &*self.weight.to_string(),
            port => &*self.port.to_string(),
            target => self.target.borrow(),

            subdomain => self.subdomain.borrow(),
            ttl => &*self.ttl.to_string(),
        })
    }
}

impl<'a> YaVisitor for EditRequest<'a> {
    type Reply = EditReply;
    fn visit(&self, api: &mut YandexDNS) -> Result<Self::Reply> {
        let record_id = self.record_id.to_string();
        let refresh = self.refresh.map(|v| v.to_string());
        let retry = self.retry.map(|v| v.to_string());
        let expire = self.expire.map(|v| v.to_string());
        let neg_cache = self.neg_cache.map(|v| v.to_string());
        let priority = self.priority.map(|v| v.to_string());
        let port = self.port.map(|v| v.to_string());
        let weight = self.weight.map(|v| v.to_string());
        let ttl = self.ttl.map(|v| v.to_string());

        api.call("edit", Method::Post, qs! {
            domain => self.domain.borrow(),
            record_id => &*record_id,

            subdomain => opt_borrow!(self.subdomain),
            ttl => opt_borrow!(ttl),
            refresh => opt_borrow!(refresh),
            retry => opt_borrow!(retry),
            expire => opt_borrow!(expire),
            neg_cache => opt_borrow!(neg_cache),
            admin_mail => opt_borrow!(self.admin_mail),
            content => opt_borrow!(self.content),
            priority => opt_borrow!(priority),
            port => opt_borrow!(port),
            weight => opt_borrow!(weight),
            target => opt_borrow!(self.target),
        })
    }
}

impl<'a> YaVisitor for DeleteRequest<'a> {
    type Reply = DeleteReply;
    fn visit(&self, api: &mut YandexDNS) -> Result<Self::Reply> {
        api.call("delete", Method::Post, qs! {
            domain => self.domain.borrow(),
            record_id => &*self.record_id.to_string(),
        })
    }
}
