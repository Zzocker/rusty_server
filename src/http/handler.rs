use super::{Request,Response};

pub trait Handler{
    fn handle(&mut self,req: &Request) -> Response;
}