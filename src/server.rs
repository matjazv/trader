use crate::site::HelloTemplate;
use askama::Template;
use http::{Response, StatusCode};
use std::error::Error;
use std::fs;
use std::sync::Arc;
use warp::Filter;

trait ForWarp {
    type Reply;

    fn for_warp(self) -> Result<Self::Reply, warp::Rejection>;
}

impl<T> ForWarp for Result<T, Box<dyn Error>>
where
    T: warp::Reply + 'static,
{
    type Reply = Box<dyn warp::Reply>;

    fn for_warp(self) -> Result<Self::Reply, warp::Rejection> {
        let b: Box<dyn warp::Reply> = match self {
            Ok(reply) => Box::new(reply),
            Err(_e) => {
                let res = http::Response::builder()
                    .status(500)
                    .body("Something went wrong, apologies.");
                Box::new(res)
            }
        };
        Ok(b)
    }
}

pub async fn server_run(user_manager: UserManager) -> Result<(), Box<dyn Error>> {
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    let hi = warp::path("hi")
        .and(warp::path::param())
        .and(warp::header("user-agent"))
        .map(|param: String, agent: String| format!("Hello {}, whose agent is {}", param, agent));

    let hello_template = HelloTemplate { name: "Matjaz" };
    //println!("{}", hello_template.render().unwrap());

    let hello_template = Arc::new(hello_template);
    let template = warp::path!("temp").map(move || {
        Response::builder()
            .status(StatusCode::OK)
            .header("content-type", "text/html; charset=utf-8")
            .body(hello_template.render().unwrap().to_string())
            .unwrap()
    });

    let style = warp::filters::method::get()
        .and(warp::path!("style.css"))
        .and_then(|| async move {
            let res = Ok(Response::builder()
                .header("content-type", "text/css; charset=utf-8")
                .body(fs::read_to_string("templates/style.css").unwrap()))
            .for_warp();
            res
        });

    let js = warp::filters::method::get()
        .and(warp::path!("main.js"))
        .and_then(|| async move {
            let res = Ok(Response::builder()
                .header("content-type", "text/javascript; charset=utf-8")
                .body(fs::read_to_string("templates/main.js").unwrap()))
            .for_warp();
            res
        });

    let login_user = warp::post()
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(warp::body::content_length_limit(1024 * 16).and(warp::body::json()))
        .and(warp::any().map(move || user_manager.clone()))
        .and_then(add_user_list_item);

    warp::serve(hello.or(hi.or(template.or(style.or(js.or(login_user))))))
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}

use crate::UserManager;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct NewUser {
    account: String,
    signature: String,
}

async fn add_user_list_item(
    item: NewUser,
    mut user_manager: UserManager,
) -> Result<impl warp::Reply, warp::Rejection> {
    user_manager.add_user(&item.account).unwrap();
    println!("user: {:?}", item);
    println!(
        "Total users in database: {}",
        user_manager.users.read().len()
    );
    for (_, user) in user_manager.users.read().iter() {
        println!("{}", user.address());
    }

    Ok(warp::reply::with_status(
        "Added new user to users list",
        http::StatusCode::CREATED,
    ))
}
