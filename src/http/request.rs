pub struct Request<'buf>{
    method: Method,
    path: &'buf str
}

pub enum Method{
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}