mod page;
mod server;
mod user;
mod user_database;
mod user_manager;
mod wallet;

use server::server_run;
use user::User;
use user_database::UserDatabase;
use user_manager::UserManager;
use wallet::get_manager;

#[tokio::main]
async fn main() -> iota_wallet::Result<()> {
    let user_database = UserDatabase {};
    let user_manager = UserManager::init(&user_database);

    let manager = get_manager().await.unwrap();
    // for (_, user) in user_manager.users.read().iter() {
    //     let account = get_account(&manager, user.account()).await.unwrap();
    //
    //     account.generate_addresses(5, None).await?;
    //     let addresses = account.list_addresses().await?;
    //     println!("Addresses: {}", addresses.len());
    //
    //     let balance = account.sync(None).await?;
    //     println!("Balance: {:?}", balance);
    // }
    let accounts = manager.get_accounts().await?;
    for account in accounts {
        let a = account.read().await;
        println!("Accounts: {:#?}", a);
    }

    let mut user = User::new("mihec");
    user.set_nick_name("pihec");
    user_database.create_tables();
    user_database.add_user(&user);
    let account = user_database.get_user("mihec").unwrap();
    println!("Found account {:?}", account);

    server_run(user_manager).await.unwrap();

    Ok(())
}
