use crate::UserManager;
use askama::Template;
use http::{Response, StatusCode};
use serde_derive::{Deserialize, Serialize};
use tokio::fs;

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
pub struct NewUser {
    account: String,
    signature: String,
}

pub async fn login_page(
    new_user: NewUser,
    mut user_manager: UserManager,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Ok(_user) = user_manager.get_user(&new_user.account) {
        let reply =
            warp::reply::with_status("Added new user to users list", http::StatusCode::CREATED);
        let cookie_value = format!("account={}; SameSite=strict", new_user.account);
        let reply = warp::reply::with_header(reply, "set-cookie", cookie_value);
        Ok(reply)
    } else {
        user_manager.add_user(&new_user.account).unwrap();
        println!("New User: {:?}", new_user);
        println!(
            "Total users in database: {}",
            user_manager.users.read().len()
        );
        for (_, user) in user_manager.users.read().iter() {
            println!("{}", user.address());
        }

        let reply =
            warp::reply::with_status("Added new user to users list", http::StatusCode::CREATED);
        let cookie_value = format!("account={}; SameSite=strict", new_user.account);
        let reply = warp::reply::with_header(reply, "set-cookie", cookie_value);
        Ok(reply)
    }
}

pub async fn logout_page(cookie: String) -> Result<impl warp::Reply, warp::Rejection> {
    let _cookie = cookie;
    let reply = warp::reply::with_status("Log out user", http::StatusCode::CREATED);
    let cookie_value = format!("account=; SameSite=strict");
    let reply = warp::reply::with_header(reply, "set-cookie", cookie_value);
    Ok(reply)
}
