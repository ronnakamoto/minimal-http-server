use std::{
    error::Error,
    fmt::Display,
    str::{from_utf8, Utf8Error},
};

use super::method::{Method, MethodError};

/// GET / HTTP/1.1
/// Host: 127.0.0.1:8080
/// User-Agent: curl/7.79.1
/// Accept: */*
#[allow(dead_code)]
#[derive(Debug)]
pub struct Request<'b> {
    /// Method
    method: Method,
    /// Path
    path: &'b str,
    /// Query string
    query_string: Option<&'b str>,
}

// impl Request {
//     pub fn from_bytes(buffer: &[u8]) -> Result<Self, String> {
//         todo!()
//     }
// }

impl<'b> TryFrom<&'b [u8]> for Request<'b> {
    type Error = RequestError;

    fn try_from(buffer: &'b [u8]) -> Result<Self, Self::Error> {
        // convert byte slice to string
        let utf8_request = from_utf8(buffer)?;
        // parse string
        let request_lines = utf8_request.split("\r\n").collect::<Vec<&str>>();
        // println!("request_lines: {:?}", request_lines);
        let req_line_0 = request_lines[0].split_whitespace().collect::<Vec<&str>>();
        let (method, query_string_raw) = (req_line_0[0], req_line_0[1]);

        // println!(
        //     "method: {}, query string: {}, http version: {}",
        //     method, query_string_raw, http_version_raw
        // );
        // construct the Request object
        // parse the method
        let method = method.parse()?;
        // parse the path and query string
        // let question_index = query_string_raw.find("?");
        let mut query_string = None;
        let mut path = "";
        // match question_index {
        //     Some(i) => {
        //         path = &query_string_raw[..i];
        //         query_string = Some(&query_string_raw[i + 1..]);
        //     }
        //     None => (),
        // }

        if let Some(question_index) = query_string_raw.find("?") {
            path = &query_string_raw[..question_index];
            query_string = Some(&query_string_raw[question_index + 1..]);
        }
        Ok(Request {
            method,
            path: path,
            query_string,
        })
    }
}

impl From<MethodError> for RequestError {
    fn from(_: MethodError) -> Self {
        RequestError::InvalidMethod
    }
}

impl From<Utf8Error> for RequestError {
    fn from(_: Utf8Error) -> Self {
        RequestError::InvalidEncoding
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum RequestError {
    InvalidMethod,
    InvalidProtocol,
    InvalidEncoding,
}

impl RequestError {
    fn msg(&self) -> &str {
        match self {
            Self::InvalidMethod => "Invalid Method",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidEncoding => "Invalid Encoding",
        }
    }
}

impl Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg())
    }
}

impl Error for RequestError {}
