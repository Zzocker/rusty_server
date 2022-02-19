use crate::http::{Handler,Request,Response};

pub struct ServiceHandler{}

impl Handler for ServiceHandler{
    fn handle(&mut self,req: &Request) -> Response{
        unimplemented!();
    }
}