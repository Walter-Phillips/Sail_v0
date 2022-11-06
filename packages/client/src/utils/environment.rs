use fuels::{
    signers::WalletUnlocked,
    test_helpers::setup_test_client
};


pub async fn setup_environment() -> (pk:&str) {
    let (provider, _address) = setup_test_provider(vec![], vec![], None, None).await;
    let wallet = WalletUnlocked::new_from_private_key(pk, Some(provider));
    (wallet, provider)
}