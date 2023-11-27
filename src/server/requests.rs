use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
enum RequestType {
    CONNECT,
    DISCONNECT,
}

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    req_type: RequestType,
}

// Handle the request based on its type
pub fn handle_request(request: &str) {
    let deserialized: Request = serde_json::from_str(request).unwrap();
    match deserialized.req_type {
        RequestType::CONNECT => {
            println!("Handle Connect");
        }
        RequestType::DISCONNECT => {
            println!("Handle Disconnect");
        }
    }
}
