use super::{Handler,Request,Response,StatusCode};
use std::net::TcpListener;
use std::io::Read;

pub struct Server{
    address:String
}

impl Server{
    pub fn new(addr: &str) -> Self{
        return Server{
            address: addr.to_string()
        };
    }

    pub fn run(&self,mut handler: impl Handler){
        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept(){
                Ok((mut stream,_)) => {
                    let mut buffer = [0; 1024]; // 1kb bytes
                    match stream.read(&mut buffer){
                        Ok(size) => {
                            println!("Received a request: {}",String::from_utf8_lossy(&buffer[..size].to_vec()));
                            let response = match Request::try_from(&buffer[..size]){
                                Ok(req) => {
                                    handler.handle(&req)
                                },
                                Err(err) => {
                                    println!("failed to parse request: {}",err);
                                    Response::new(StatusCode::BadRequest,None)
                                }
                            };

                            if let Err(err) = response.send(&mut stream){
                                println!("failed to send response: {}",err);
                            }
                        },
                        Err(err) => println!("failed to read data from connection: {}",err) 
                    }
                },
                Err(err) => println!("failed to establish connection: {}",err)
            }
        }
    }
}