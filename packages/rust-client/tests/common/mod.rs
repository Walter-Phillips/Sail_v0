use fuels::prelude::*;

use fuels::{
    signers::WalletUnlocked,
    test_helpers::{setup_test_provider, Config},
    tx::{Address, AssetId},
};

use fuels_signers::provider::Provider;

use fuel_core_interfaces::common::fuel_crypto::SecretKey;

pub async fn setup() -> (
    Provider,
    WalletUnlocked,
    Address,
    WalletUnlocked,
    Address,
    WalletUnlocked,
    Address,
) {
    let address1 = Address::zeroed();
    let address2 = Address::zeroed();
    let address3 = Address::zeroed();

    let secret_key1: SecretKey =
        "0x862512a2363db2b3a375c0d4bbbd27172180d89f23f2e259bac850ab02619301"
            .parse()
            .unwrap();

    let secret_key2: SecretKey =
        "0x37fa81c84ccd547c30c176b118d5cb892bdb113e8e80141f266519422ef9eefd"
            .parse()
            .unwrap();

    let secret_key3: SecretKey =
        "0x976e5c3fa620092c718d852ca703b6da9e3075b9f2ecb8ed42d9f746bf26aafb"
            .parse()
            .unwrap();

    let mut wallet = WalletUnlocked::new_from_private_key(secret_key1, None);
    let mut wallet2 = WalletUnlocked::new_from_private_key(secret_key2, None);
    let mut wallet3 = WalletUnlocked::new_from_private_key(secret_key3, None);

    let all_coins = [&wallet, &wallet2, &wallet3]
        .iter()
        .flat_map(|wallet| {
            setup_single_asset_coins(wallet.address(), AssetId::default(), 10, 1_000_000)
        })
        .collect::<Vec<_>>();

    let (provider, _) = setup_test_provider(
        all_coins,
        vec![],
        Some(Config {
            utxo_validation: true,
            ..Config::local_node()
        }),
    )
    .await;

    [&mut wallet, &mut wallet2, &mut wallet3]
        .iter_mut()
        .for_each(|wallet| wallet.set_provider(provider.clone()));

    (
        provider, wallet, address1, wallet2, address2, wallet3, address3,
    )
}
