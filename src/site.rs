use crate::UserManager;
use askama::Template;
use http::{Response, StatusCode};
use serde_derive::{Deserialize, Serialize};
use tokio::fs;

pub async fn hello_page(name: String) -> Result<impl warp::Reply, warp::Rejection> {
    let hello = format!("Hello, {}!", name);
    Ok(warp::reply::with_status(hello, http::StatusCode::OK))
}

pub async fn hi_page(name: String, agent: String) -> Result<impl warp::Reply, warp::Rejection> {
    let hello = format!("Hello {}, whose agent is {}", name, agent);
    Ok(warp::reply::with_status(hello, http::StatusCode::OK))
}

pub async fn style_page() -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let res = Response::builder()
        .header("content-type", "text/css; charset=utf-8")
        .body(fs::read_to_string("templates/style.css").await.unwrap());
    let res: Box<dyn warp::Reply> = Box::new(res);
    Ok(res)
}

pub async fn js_page() -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let res = Response::builder()
        .header("content-type", "text/javascript; charset=utf-8")
        .body(fs::read_to_string("templates/main.js").await.unwrap());
    let res: Box<dyn warp::Reply> = Box::new(res);
    Ok(res)
}

pub async fn check_cookie_page(cookie: String) -> Result<impl warp::Reply, warp::Rejection> {
    let hello = format!("Cookie value: {}", cookie);
    Ok(warp::reply::with_status(hello, http::StatusCode::OK))
}

#[derive(Template)]
#[template(path = "hello.html")]
pub struct HelloTemplate<'a> {
    pub name: &'a str,
}

pub async fn template_page() -> Result<impl warp::Reply, warp::Rejection> {
    let hello_template = HelloTemplate { name: "Matjaz V." };

    let res = Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/html; charset=utf-8")
        .body(hello_template.render().unwrap())
        .unwrap();

    let res: Box<dyn warp::Reply> = Box::new(res);
    Ok(res)
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NewUser {
    account: String,
    signature: String,
}

pub async fn add_user_page(
    new_user: NewUser,
    mut user_manager: UserManager,
) -> Result<impl warp::Reply, warp::Rejection> {
    user_manager.add_user(&new_user.account).unwrap();
    println!("New User: {:?}", new_user);
    println!(
        "Total users in database: {}",
        user_manager.users.read().len()
    );
    for (_, user) in user_manager.users.read().iter() {
        println!("{}", user.address());
    }

    let reply = warp::reply::with_status("Added new user to users list", http::StatusCode::CREATED);
    let value = format!("address={}", new_user.account);
    let reply = warp::reply::with_header(reply, "set-cookie", value);
    Ok(reply)
}
