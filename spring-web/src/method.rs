/// HTTP 方法枚举
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
}

impl HttpMethod {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "GET"     => Some(Self::GET),
            "POST"    => Some(Self::POST),
            "PUT"     => Some(Self::PUT),
            "DELETE"  => Some(Self::DELETE),
            "PATCH"   => Some(Self::PATCH),
            "HEAD"    => Some(Self::HEAD),
            "OPTIONS" => Some(Self::OPTIONS),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::GET     => "GET",
            Self::POST    => "POST",
            Self::PUT     => "PUT",
            Self::DELETE  => "DELETE",
            Self::PATCH   => "PATCH",
            Self::HEAD    => "HEAD",
            Self::OPTIONS => "OPTIONS",
        }
    }
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
