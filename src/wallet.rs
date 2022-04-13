use iota_wallet::{
    account::AccountHandle, account_manager::AccountManager, signing::mnemonic::MnemonicSigner,
    ClientOptions,
};

pub async fn get_manager() -> iota_wallet::Result<AccountManager> {
    let client_options = ClientOptions::new()
        .with_node("https://api.alphanet.iotaledger.net")?
        .with_node_sync_disabled();

    let signer = MnemonicSigner::new("obvious wild country kind relief social barrel front visual nephew gaze voice pelican mechanic galaxy ecology squeeze fuel pet subject random need bulk jealous")?;
    //use iota_wallet::iota_client::Client;
    //let mnemonic = Client::generate_mnemonic()?;
    //println!("Mnemonic: {}", mnemonic);
    //let signer = MnemonicSigner::new(&mnemonic)?;

    let manager = AccountManager::builder()
        .with_signer(signer)
        .with_client_options(client_options)
        .finish()
        .await?;

    Ok(manager)
}

#[allow(dead_code)]
pub async fn get_account(
    manager: &AccountManager,
    address: &str,
) -> iota_wallet::Result<AccountHandle> {
    println!("Getting account for address: {}", address);

    // Get account or create a new one
    let account = match manager.get_account(address).await {
        Ok(account) => account,
        _ => {
            // first we'll create an example account and store it
            manager
                .create_account()
                .with_alias(address.to_string())
                .finish()
                .await?
        }
    };

    Ok(account)
}
