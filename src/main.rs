extern crate antidote;
extern crate futures;
extern crate futures_cpupool;
extern crate grpc;
extern crate httpbis;
extern crate protobuf;
extern crate tls_api;

pub mod helloworld;
pub mod helloworld_grpc;

use std::thread;
use std::sync::Arc;
use std::time::Duration;

use antidote::Mutex;

use grpc::Client;

use helloworld::*;
use helloworld_grpc::*;

struct Greeting {
    pub inner: String,
}

struct HelloServer {
    pub preferred_greeting: Arc<Mutex<Greeting>>,
}

impl HelloServer {
    pub fn new() -> Self {
        let greeting = String::from("Hello, ");
        let preferred_greeting = Arc::new(Mutex::new(Greeting { inner: greeting }));
        return HelloServer { preferred_greeting };
    }
}

impl HelloWorld for HelloServer {
    fn hello_message(
        &self,
        _m: grpc::RequestOptions,
        args: HelloMessageArgs,
    ) -> grpc::SingleResponse<HelloMessageReply> {
        let mut reply = HelloMessageReply::new();
        let name: String = if args.get_name().is_empty() {
            "World".to_string()
        } else {
            args.get_name().to_string()
        };
        let shared_data = self.preferred_greeting.lock();
        reply.set_greeting(shared_data.inner.to_owned() + &name);
        grpc::SingleResponse::completed(reply)
    }
}

fn main() {
    let my_server = HelloServer::new();
    let shared_data = Arc::clone(&(my_server.preferred_greeting));

    let conf = httpbis::ServerConf::default();
    let mut server = grpc::ServerBuilder::new_plain();
    server.http.conf = conf.clone();
    server.http.set_port(8080);
    server.add_service(HelloWorldServer::new_service_def(my_server));
    let _server = server.build().expect("Failed to build server");

    
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(10));
        let mut greeting = shared_data.lock();
        greeting.inner = "Gutentag, ".to_string();
    });

    thread::spawn(|| {
        ping_server();
    });

    loop {
        thread::park();
    }
}

fn ping_server() {
    let client = Client::new_plain("::1", 8080, Default::default())
        .expect("Failed to initialize client");
    let hello_client = HelloWorldClient::with_client(client);

    let mut i = 0;
    loop {
        let mut args = HelloMessageArgs::new();
        args.set_name(format!("World-{}", i));
        i += 1;
        let resp = hello_client.hello_message(grpc::RequestOptions::default(), args);
        match resp.wait() {
            Ok(resp) => {
                println!("{:?}", resp.1.greeting);
            }
            _ => {}
        }
        thread::sleep(Duration::from_millis(1000));
    }
}
