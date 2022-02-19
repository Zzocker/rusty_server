use crate::http::{Handler,Request,Response,StatusCode,Method};

pub struct ServiceHandler{}

impl Handler for ServiceHandler{
    fn handle(&mut self,req: &Request) -> Response{
        return match req.method(){
            Method::GET => match req.path(){
                "/" => Response::new(StatusCode::Ok,Some("Hello".to_string())),
                "/ping" => Response::new(StatusCode::Ok,Some("pong".to_string())),
                _ => Response::new(StatusCode::NotImplemented, None)  
            }
            _ => Response::new(StatusCode::NotImplemented, None)  
        };
        
    }
}