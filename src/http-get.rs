use std::env;

extern crate hyper;
extern crate hyper_rustls;
use hyper::client::Client;
use hyper::net::HttpsConnector;

fn main() {
    let url = env::args().nth(1).unwrap();
    println!("Retrieving: {:?}", url);

    let client = Client::with_connector(HttpsConnector::new(hyper_rustls::TlsClient::new()));
    match client.get(&url).send() {
        Ok(response) => println!("{:?}", response),
        Err(error) => println!("{:?}", error),
    }
}
