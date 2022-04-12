use crate::UserManager;
use askama::Template;
use http::{Response, StatusCode};
use serde_derive::{Deserialize, Serialize};
use tokio::fs;

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

pub async fn login_page(
    login_user: LoginUser,
    mut user_manager: UserManager,
) -> Result<impl warp::Reply, warp::Rejection> {
    if user_manager.get_user(&login_user.account).is_none() {
        user_manager.add_user(&login_user.account).unwrap();
        println!("New User: {:?}", login_user);
        println!(
            "Total users in database: {}",
            user_manager.users.read().len()
        );
        for (_, user) in user_manager.users.read().iter() {
            println!("{}", user.account());
        }
    }

    let cookie_value = format!("account={}; SameSite=strict", login_user.account);
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("set-cookie", cookie_value)
        .body("");
    let response: Box<dyn warp::Reply> = Box::new(response);
    Ok(response)
}

pub async fn logout_page() -> Result<impl warp::Reply, warp::Rejection> {
    let cookie_value = "account=; SameSite=strict".to_string();
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("set-cookie", cookie_value)
        .body("");
    let response: Box<dyn warp::Reply> = Box::new(response);
    Ok(response)
}
