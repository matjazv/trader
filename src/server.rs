use crate::page::{add_user_page, js_page, log_out_page, main_page, style_page};
use crate::UserManager;
use std::error::Error;
use warp::Filter;

pub async fn server_run(user_manager: UserManager) -> Result<(), Box<dyn Error>> {
    let style = warp::filters::method::get()
        .and(warp::path!("style.css"))
        .and(warp::path::end())
        .and_then(style_page);

    let js = warp::filters::method::get()
        .and(warp::path!("main.js"))
        .and(warp::path::end())
        .and_then(js_page);

    let main = warp::path::end()
        .and((warp::cookie("account")).or(warp::any().map(|| String::default())))
        .unify()
        .and_then(main_page);

    let login_user = warp::post()
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(warp::body::content_length_limit(1024 * 16).and(warp::body::json()))
        .and(warp::any().map(move || user_manager.clone()))
        .and_then(add_user_page);

    let log_out = warp::path("logout")
        .and(warp::path::end())
        .and(warp::cookie("account"))
        .and_then(log_out_page);

    warp::serve(style.or(js.or(main.or(login_user.or(log_out)))))
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}
