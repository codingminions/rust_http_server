use super::method::{Method, MethodError};
use super::QueryString;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }
    pub fn method(&self) -> &Method {
        &self.method
    }
    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let request = str::from_utf8(buf)?;
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;
        let mut query_string = None;

        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (idx, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            // Adding idx + 1 is dangerous in Rust, because they are utf8 encoded.
            return Some((&request[..idx], &request[idx + 1..]));
        }
    }
    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}

// Display trait(invoked via {} formatting) is used when we are formatting our string line for say printing.
// Debug trait is invoked via {:?} formatting.

// match path.find('?') {
//     Some(i) => {
//         query_string = Some(&path[i + 1..]);
//         path = &path[..i];
//     }
//     None => {}
// }

// let q = path.find('?');
// if q.is_some() {
//     let i = q.unwrap();
//     query_string = Some(&path[i + 1..]);
//     path = &path[..i];
// }

// GET /search?name=abc&sort=1 HTTP/1.1
// fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
// match str::from_utf8(buf) {
//     Ok(request) => {}
//     Err(_) => return Err(ParseError::InvalidEncoding),
// }

// match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
//     Ok(request) => {}
//     Err(e) => return Err(e),
// }

// let request = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;
// let request = str::from_utf8(buf)?;
// let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
// let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
// let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

// if protocol != "HTTP/1.1" {
//     return Err(ParseError::InvalidProtocol);
// }
// code snippet of using Some/None spec.
// match get_next_word(request) {
//     Some((method, request)) => {}
//     None => return Err(ParseError::InvalidRequest),
// }

// iterating through a string
// let mut iter = request.chars();
// loop {
//     let item = iter.next();
//     match item {
//         Some(c) => {}
//         None => break,
//     }
// }
