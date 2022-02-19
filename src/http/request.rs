use std::convert::TryFrom;
use std::str;
use std::str::{FromStr};
use std::fmt::{Display,Result as FmtResult,Formatter};

#[derive(Debug)]
pub struct Request<'buf>{
    method: Method,
    path: &'buf str
}

impl<'buf> Request<'buf>{
    pub fn method(&self) -> &Method{
        return &self.method;
    }

    pub fn path(&self) -> &'buf str{
        return self.path;
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf>{
    type Error = ParseError;
    
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>,Self::Error>{
        let request = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;
        // POST /path HTTP/1.1
        let (method,request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path,request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?; 
        let (protocol,_) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1"{
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;
        
        if let Some(index) = path.find('?'){
            path = &path[..index];
        }

        return Ok(Request{
            method: method,
            path: path
        });
    }
}

pub enum ParseError{
    InvalidEncoding,
    InvalidRequest,
    InvalidProtocol,
    InvalidMethod,
}

fn get_next_word<'a>(input: &'a str) -> Option<(&'a str,&'a str)>{
    for (index,c) in input.chars().enumerate(){
        if c == ' ' || c == '\r'{
            return Some((&input[..index],&input[index+1..]))
        }
    }
    return None;
}

impl Display for ParseError{
    fn fmt(&self,f: &mut Formatter<'_>) -> FmtResult{
        write!(f,"{}",self.message());
        return Ok(());
    }
}

impl ParseError{
    fn message(&self) -> &str{
        return match self{
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

#[derive(Debug)]
pub enum Method{
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

impl FromStr for Method{
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self,Self::Err>{
        return match s {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            "PATCH" => Ok(Method::PATCH),
            _ => Err(ParseError::InvalidMethod),
        };
    }
}