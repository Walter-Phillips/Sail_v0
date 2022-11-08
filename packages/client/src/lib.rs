//test
mod utils {
    pub mod build_take_order;
    pub mod environment;
    pub mod order;
    pub mod create_predicate;
}

use utils::{create_predicate};
use fuels::prelude::*;
use fuels::{
    tx::{Address, AssetId, Input, Output, Receipt, Transaction, TxPointer, UtxoId, Word, ContractId},
    test_helpers::{setup_custom_assets_coins, setup_test_provider, Config},
};
use fuel_core_interfaces::common::fuel_crypto::SecretKey;

    #[tokio::test]
    async fn test_predicate_creation() {
    let address0 = Address::zeroed();
    let address1 = Address::zeroed();
    let address2 = Address::zeroed();
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
    let receiver = WalletUnlocked::new_random(None);

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
            })
        )
        .await;

        [&mut wallet, &mut wallet2, &mut wallet3]
            .iter_mut()
            .for_each(|wallet| wallet.set_provider(provider.clone()));


        let predicate = create_predicate::create_predicate("0x7895d0059c0d0c1de8de15795191a1c1d01cd970db75fa42e15dc96e051b5570".to_string(),"1_000_000".to_string(),"0u8".to_string(),address0,"12312323".to_string(),"23131231".to_string(),address0,address2,"123123".to_string());

        let predicate_code = predicate.code();
        let predicate_address  = predicate.address();
        let amount_to_predicate = 1000;
        let asset_id = AssetId::default();

        wallet
            .transfer(
                predicate_address,
                amount_to_predicate,
                asset_id,
                TxParameters::default(),
            )
            .await;

        let predicate_balance = provider
            .get_asset_balance(predicate.address(), asset_id)
            .await;
        assert_eq!(predicate_balance.unwrap(), amount_to_predicate);
    }
    

