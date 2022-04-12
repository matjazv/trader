// site.rs
pub async fn hello_page(name: String) -> Result<impl warp::Reply, warp::Rejection> {
    let hello = format!("Hello, {}!", name);
    Ok(warp::reply::with_status(hello, http::StatusCode::OK))
}

pub async fn hi_page(name: String, agent: String) -> Result<impl warp::Reply, warp::Rejection> {
    let hello = format!("Hello {}, whose agent is {}", name, agent);
    Ok(warp::reply::with_status(hello, http::StatusCode::OK))
}

pub async fn check_cookie_page(cookie: String) -> Result<impl warp::Reply, warp::Rejection> {
    let hello = format!("Cookie value: {}", cookie);
    Ok(warp::reply::with_status(hello, http::StatusCode::OK))
}

// server.rs
// let hello = warp::path!("hello" / String)
// .and(warp::path::end())
// .and_then(hello_page);
//
// let hi = warp::path("hi")
// .and(warp::path::param())
// .and(warp::header("user-agent"))
// .and_then(hi_page);
//
// let check_cookie = warp::path("cookie")
// .and(warp::path::end())
// .and(warp::cookie("address"))
// .and_then(check_cookie_page);
