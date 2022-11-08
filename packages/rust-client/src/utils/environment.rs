// use fuels::{
//     signers::WalletUnlocked,
//     prelude::Provider
// };
// use fuel_core_interfaces::common::fuel_crypto::SecretKey;


// pub async fn setup_environment(pk:SecretKey) -> (WalletUnlocked, Provider){
//     let (provider, _address) = setup_test_provider(vec![], vec![], None, None).await;
//     let wallet = WalletUnlocked::new_from_private_key(pk, Some(provider));
//     (wallet, provider)
// }