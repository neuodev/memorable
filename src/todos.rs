use actix_web::dev::Payload;
use actix_web::error::ErrorBadRequest;
use actix_web::{Error, FromRequest};
use futures_util::future::{ready, Ready};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub desc: String,
    pub is_done: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTodo {
    pub title: String,
    pub desc: String,
    pub is_done: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub desc: Option<String>,
    pub is_done: Option<bool>,
}

#[derive(Debug)]
pub struct SocketIPAddr(String);

impl FromRequest for SocketIPAddr {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut Payload) -> Self::Future {
        match req.peer_addr() {
            Some(addr) => ready(Ok(SocketIPAddr(addr.ip().to_string()))),
            None => ready(Err(ErrorBadRequest("Missing client IP address"))),
        }
    }
}
