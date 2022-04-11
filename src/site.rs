use askama::Template;
use http::{Response, StatusCode};
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
