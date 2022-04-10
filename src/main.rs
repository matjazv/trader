mod server;
mod site;
mod user;
mod user_manager;
mod wallet;

use askama::Template;
use http::{Response, StatusCode};
use iota_wallet::iota_client::api::transaction::sign_transaction;
use site::HelloTemplate;
use std::error::Error;
use std::fs;
use std::sync::Arc;
use user::User;
use wallet::{get_account, get_manager};
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
            Err(e) => {
                let res = http::Response::builder()
                    .status(500)
                    .body("Something went wrong, apologies.");
                Box::new(res)
            }
        };
        Ok(b)
    }
}

#[tokio::main]
async fn main() -> iota_wallet::Result<()> {
    let users = vec![
        User::new("0x8bb449c7cc4a7822da23a0ffdc27de1f935681ef".to_string()),
        User::new("0xf1f758A55B8Ac233a4B0573E21783E131E4d2719".to_string()),
    ];

    let manager = get_manager().await.unwrap();
    for user in users {
        let account = get_account(&manager, user).await.unwrap();

        account.generate_addresses(5, None).await?;
        let addresses = account.list_addresses().await?;
        println!("Addresses: {}", addresses.len());

        let balance = account.sync(None).await?;
        println!("Balance: {:?}", balance);
    }
    let accounts = manager.get_accounts().await?;
    for account in accounts {
        let a = account.read().await;
        println!("Accounts: {:#?}", a);
    }

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

    // POST /login/:rate  {"name":"Sean","rate":2}
    // let login_user = warp::post()
    //     .and(warp::path("login"))
    //     .and(warp::path::param::<String>())
    //     // Only accept bodies smaller than 16kb...
    //     .and(warp::body::content_length_limit(1024 * 16))
    //     .map(|signed_message: String| {
    //         println!("{}", signed_message);
    //         //warp::reply::json()
    //         warp::reply::html("test")
    //     });

    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let login_user = warp::post()
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(warp::body::content_length_limit(1024 * 16).and(warp::body::json()))
        .and(store_filter.clone())
        .and_then(add_grocery_list_item);

    warp::serve(hello.or(hi.or(template.or(style.or(js.or(login_user))))))
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}

use parking_lot::RwLock;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

type Items = HashMap<String, i32>;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Item {
    name: String,
    quantity: i32,
}

#[derive(Clone)]
struct Store {
    grocery_list: Arc<RwLock<Items>>,
}

impl Store {
    fn new() -> Self {
        Store {
            grocery_list: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

async fn add_grocery_list_item(
    item: Item,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    store.grocery_list.write().insert(item.name, item.quantity);

    Ok(warp::reply::with_status(
        "Added items to the grocery list",
        http::StatusCode::CREATED,
    ))
}
