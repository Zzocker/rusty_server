mod http;
mod service;

use http::Server;
use service::ServiceHandler;

fn main() {

    Server::new("127.0.0.1:8080").run(ServiceHandler{});    
}
