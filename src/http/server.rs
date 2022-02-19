use super::Handler;

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
        unimplemented!();
    }
}