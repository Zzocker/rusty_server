pub struct Response{
    status_code: StatusCode,
    body: Option<String>
}

pub enum StatusCode{
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}