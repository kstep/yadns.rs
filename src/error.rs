
#[derive(Debug)]
enum Error {
    /// HTTP error
    Http(HttpError),
    /// JSON decode error
    Json(serde_json::error::Error),
    /// Yandex API error
    Api(ErrorReplyDTO),
    /// IO error
    Io(IoError)
}

type Result<T> = StdResult<T, Error>;

impl StdError for Error {
    fn description(&self) -> &str {
        use Error::*;
        match *self {
            Http(ref err) => err.description(),
            Json(ref err) => err.description(),
            Api(ref err) => err.description(),
            Io(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        use Error::*;
        Some(match *self {
            Http(ref err) => err,
            Json(ref err) => err,
            Api(ref err) => err,
            Io(ref err) => err,
        })
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match *self {
            Http(ref err) => err.fmt(f),
            Json(ref err) => err.fmt(f),
            Api(ref err) => err.fmt(f),
            Io(ref err) => err.fmt(f),
        }
    }
}

impl From<HttpError> for Error {
    fn from(value: HttpError) -> Error {
        match value {
            HttpError::Io(err) => Error::Io(err),
            _ => Error::Http(value)
        }
    }
}
impl From<::serde_json::error::Error> for Error {
    fn from(value: ::serde_json::error::Error) -> Error {
        Error::Json(value)
    }
}
impl From<ErrorReplyDTO> for Error {
    fn from(value: ErrorReplyDTO) -> Error {
        Error::Api(value)
    }
}
impl From<IoError> for Error {
    fn from(value: IoError) -> Error {
        Error::Io(value)
    }
}

impl StdError for ErrorReplyDTO {
    fn description(&self) -> &str {
        self.error.description()
    }
}

impl fmt::Display for ErrorReplyDTO {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(f)
    }
}

#[derive(Debug, Clone)]
enum ErrorCode {
    Unknown,
    NoToken,
    NoDomain,
    NoContent,
    NoType,
    NoIp,
    BadDomain,
    Prohibited,
    BadToken,
    BadLogin,
    BadPasswd,
    NoAuth,
    NotAllowed,
    Blocked,
    Occupied,
    DomainLimitReached,
    NoReply,
}

impl StdError for ErrorCode {
    fn description(&self) -> &str {
        use ErrorCode::*;
        match *self {
            Unknown => "unknown error",
            NoToken => "access token missing",
            NoDomain => "domain name missing",
            NoContent => "content missing",
            NoType => "type missing",
            NoIp => "IP address missing",
            BadDomain => "invalid domain name",
            Prohibited => "domain name forbidden",
            BadToken => "invalid token",
            BadLogin => "invalid login",
            BadPasswd => "invalid password",
            NoAuth => "authorization missing",
            NotAllowed => "access denied",
            Blocked => "domain name blocked",
            Occupied => "domain name occupied",
            DomainLimitReached => "max number of domains exceeded",
            NoReply => "server access error",
        }
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(f)
    }
}

impl serde::Deserialize for ErrorCode {
    fn deserialize<D: serde::Deserializer>(d: &mut D) -> StdResult<ErrorCode, D::Error> {
        struct ErrorCodeVisitor;

        impl serde::de::Visitor for ErrorCodeVisitor {
            type Value = ErrorCode;
            fn visit_str<E: serde::de::Error>(&mut self, v: &str) -> StdResult<ErrorCode, E> {
                use self::ErrorCode::*;
                match v {
                    "unknown" => Ok(Unknown),
                    "no_token" => Ok(NoToken),
                    "no_domain" => Ok(NoDomain),
                    "no_content" => Ok(NoContent),
                    "no_type" => Ok(NoType),
                    "no_ip" => Ok(NoIp),
                    "bad_domain" => Ok(BadDomain),
                    "prohibited" => Ok(Prohibited),
                    "bad_token" => Ok(BadToken),
                    "bad_login" => Ok(BadLogin),
                    "bad_password" => Ok(BadPasswd),
                    "no_auth" => Ok(NoAuth),
                    "not_allowed" => Ok(NotAllowed),
                    "blocked" => Ok(Blocked),
                    "occupied" => Ok(Occupied),
                    "domain_limit_reached" => Ok(DomainLimitReached),
                    "no_reply" => Ok(NoReply),
                    _ => Err(serde::de::Error::unknown_field("invalid error code"))
                }
            }
        }

        d.visit_str(ErrorCodeVisitor)
    }
}
