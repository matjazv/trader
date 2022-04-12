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
use wallet::{get_account, get_manager};

#[tokio::main]
async fn main() -> iota_wallet::Result<()> {
    let mut user_manager = UserManager::init();
    user_manager
        .add_user("0x8bb449c7cc4a7822da23a0ffdc27de1f935681ef")
        .unwrap();
    user_manager
        .add_user("0x5Ac233a4B0573E21783E131E4d27199356e7fdda")
        .unwrap();

    let manager = get_manager().await.unwrap();
    for (_, user) in user_manager.users.read().iter() {
        let account = get_account(&manager, user.account()).await.unwrap();

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

    let mut user = User::new("Mihec");
    user.set_nick_name("Pihec");
    let user_database = UserDatabase::connect().unwrap();
    user_database.create_table();
    user_database.add_user(user);
    user_database.get_user("Mihec");

    server_run(user_manager).await.unwrap();

    Ok(())
}
