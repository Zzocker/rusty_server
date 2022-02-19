use std::io::{Write,Result as IOResult};
use std::fmt::{Display,Result as FmtResult,Formatter};

pub struct Response{
    status_code: StatusCode,
    body: Option<String>
}

impl Response{
    pub fn new(status_code: StatusCode,body: Option<String>) -> Self {
        return Response{
            status_code: status_code,
            body : body,
        };
    }

    pub fn send(&self,stream: &mut impl Write) -> IOResult<()>{
        let body = match &self.body{
            Some(value) => value,
            None => "",
        };
        
        write!(stream,"HTTP/1.1 {} {}\r\n\r\n{}",self.status_code,self.status_code.status_text(),body);
        return Ok(());
    } 
}

#[derive(Clone,Copy)]
pub enum StatusCode{
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
    NotImplemented = 501,
}

impl StatusCode{
    fn status_text(&self) -> &str{
        return match self{
            Self::Ok => "Ok",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
            Self::NotImplemented => "Not Implemented",
        };
    }
}

impl Display for StatusCode{
    fn fmt(&self,f : &mut Formatter) -> FmtResult{
        write!(f,"{}",*self as u16);
        return Ok(());
    }
}