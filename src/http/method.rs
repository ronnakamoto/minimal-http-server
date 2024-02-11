use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    PATCH,
    PUT,
    DELETE,
    OPTIONS,
}

impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let method_name = match s {
            "GET" => Self::GET,
            "POST" => Self::POST,
            "PATCH" => Self::PATCH,
            "PUT" => Self::PUT,
            "DELETE" => Self::DELETE,
            "OPTIONS" => Self::OPTIONS,
            _ => return Err(MethodError),
        };
        Ok(method_name)
    }
}

pub struct MethodError;
