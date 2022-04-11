use crate::site::{
    add_user_page, check_cookie_page, hello_page, hi_page, js_page, style_page, template_page,
};
use crate::UserManager;
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
        .and_then(add_user_page);

    let check_cookie = warp::path("cookie")
        .and(warp::path::end())
        .and(warp::cookie("address"))
        .and_then(check_cookie_page);

    warp::serve(hello.or(hi.or(template.or(style.or(js.or(login_user.or(check_cookie)))))))
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}
