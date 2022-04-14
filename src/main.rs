pub(crate) mod database;
mod page;
mod server;
mod user;
mod user_manager;
mod wallet;

use crate::database::Database;
use database::user_table::UserTable;
use server::server_run;
use user::User;
use user_manager::UserManager;
use wallet::get_manager;

#[tokio::main]
async fn main() -> iota_wallet::Result<()> {
    let database = Database::init();
    let user_table = UserTable::new(database);
    let user_manager = UserManager::new(&user_table);

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

    server_run(user_manager).await.unwrap();

    Ok(())
}
