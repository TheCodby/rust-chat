use std::io::Error;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
enum RequestType {
    CONNECT,
    DISCONNECT,
    #[serde(other)]
    Unknown,
}

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    req_type: RequestType,
}
fn connect() -> Result<String, Error> {
    Ok(String::from("Connected"))
}
fn disconnect() -> Result<String, Error> {
    Ok(String::from("Disconnected"))
}
// Handle the request based on its type
pub fn handle_request(request: &str) -> String {
    let deserialized: Request = serde_json::from_str(request).unwrap();
    match deserialized.req_type {
        RequestType::CONNECT => {
            println!("Handle Connect");
            connect().unwrap()
        }
        RequestType::DISCONNECT => {
            println!("Handle Disconnect");
            disconnect().unwrap()
        }
        RequestType::Unknown => {
            println!("Handle Unknown");
            "Unknown Request".to_string()
        }
    }
}
