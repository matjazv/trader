use crate::site::{check_cookie_page, hello_page, hi_page, js_page, style_page, template_page};
use std::error::Error;
use warp::Filter;

pub async fn server_run(user_manager: UserManager) -> Result<(), Box<dyn Error>> {
    let hello = warp::path!("hello" / String)
        .and(warp::path::end())
        .and_then(hello_page);

    let hi = warp::path("hi")
        .and(warp::path::param())
        .and(warp::header("user-agent"))
        .and_then(hi_page);

    let template = warp::path!("temp")
        .and(warp::path::end())
        .and_then(template_page);

    let style = warp::filters::method::get()
        .and(warp::path!("style.css"))
        .and(warp::path::end())
        .and_then(style_page);

    let js = warp::filters::method::get()
        .and(warp::path!("main.js"))
        .and(warp::path::end())
        .and_then(js_page);

    let login_user = warp::post()
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(warp::body::content_length_limit(1024 * 16).and(warp::body::json()))
        .and(warp::any().map(move || user_manager.clone()))
        .and_then(add_user_list_item);

    let check_cookie = warp::path("cookie")
        .and(warp::path::end())
        .and(warp::cookie("address"))
        .and_then(check_cookie_page);

    warp::serve(hello.or(hi.or(template.or(style.or(js.or(login_user.or(check_cookie)))))))
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

    let reply = warp::reply::with_status("Added new user to users list", http::StatusCode::CREATED);
    let value = format!("address={}", item.account);
    let reply = warp::reply::with_header(reply, "set-cookie", value);
    Ok(reply)
}
