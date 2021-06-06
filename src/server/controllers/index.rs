use crate::server::{config::ServerConfig, guards::ClientInfo};
use rocket::{http::Status, serde::json::Value, State};

#[get("/")]
pub async fn hello_world(client_info: ClientInfo) -> (Status, Value) {
    //! Hello World endpoint with remote client ip info
    json_response!(
        "message" => "Hello World!",
        "your ip" => client_info.ip
    )
}

#[get("/<name>")]
pub async fn hello_name(
    name: &str,                      // parameter
    client_info: ClientInfo,         // get client guard information
    _settings: &State<ServerConfig>, // Get the Server config from settings
) -> (Status, Value) {
    //! Hello <name> endpoint with remote client ip & browser info
    json_response!(
        "message" => format!("Hello {}!", name),
        "your ip" => client_info.ip,
        "your browser" => client_info.user_agent
    )
}
