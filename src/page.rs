use crate::server::{Session, SessionManager};
use crate::{User, UserManager};
use askama::Template;
use http::{Response, StatusCode};
use parking_lot::RwLock;
use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::fs;
use uuid::Uuid;
use web3::signing::{keccak256, recover};

pub async fn style_page() -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/css; charset=utf-8")
        .body(fs::read_to_string("templates/style.css").await.unwrap());
    let response: Box<dyn warp::Reply> = Box::new(response);
    Ok(response)
}

pub async fn js_page() -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/javascript; charset=utf-8")
        .body(fs::read_to_string("templates/main.js").await.unwrap());
    let response: Box<dyn warp::Reply> = Box::new(response);
    Ok(response)
}

#[derive(Template)]
#[template(path = "main.html")]
pub struct MainPageTemplate<'a> {
    pub account: &'a str,
}

pub async fn main_page(account: String) -> Result<impl warp::Reply, warp::Rejection> {
    let main_page_template = MainPageTemplate { account: &account };

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/html; charset=utf-8")
        .body(main_page_template.render().unwrap())
        .unwrap();

    let response: Box<dyn warp::Reply> = Box::new(response);
    Ok(response)
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoginUser {
    account: String,
    signature: String,
}

fn eth_message(message: String) -> [u8; 32] {
    keccak256(
        format!(
            "{}{}{}",
            "\x19Ethereum Signed Message:\n",
            message.len(),
            message
        )
        .as_bytes(),
    )
}

pub async fn login_page(
    login_user: LoginUser,
    mut user_manager: UserManager,
    session_manager: Arc<RwLock<SessionManager>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let message = "Please sign message.";
    let message = eth_message(message.to_string());
    let signature = hex::decode(&login_user.signature[2..]).unwrap();

    let pubkey = recover(&message, &signature[..64], 0);
    let pubkey = pubkey.unwrap();
    let pubkey = format!("{:02X?}", pubkey);
    if pubkey != login_user.account {
        println!("User failed to sign a message!");

        let response = Response::builder()
            .status(StatusCode::OK)
            .header("set-cookie", "uuid=; SameSite=strict")
            .body("");
        let response: Box<dyn warp::Reply> = Box::new(response);
        return Ok(response);
    }

    if user_manager.get_user(&login_user.account).is_none() {
        let new_user = User::new(&login_user.account, "");
        user_manager.add_user(&new_user).unwrap();
        println!("New User: {:?}", login_user);
    }

    let uuid = Uuid::new_v4();
    let new_session = Session {
        account: login_user.account,
        uuid,
    };
    session_manager.write().insert(new_session);

    let cookie_value = format!("uuid={}; SameSite=strict", uuid);
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("set-cookie", cookie_value)
        .body("");
    let response: Box<dyn warp::Reply> = Box::new(response);
    Ok(response)
}

pub async fn logout_page() -> Result<impl warp::Reply, warp::Rejection> {
    let cookie_value = "uuid=; SameSite=strict".to_string();
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("set-cookie", cookie_value)
        .body("");
    let response: Box<dyn warp::Reply> = Box::new(response);
    Ok(response)
}
