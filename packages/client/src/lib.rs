//test
mod utils {
    pub mod build_take_order;
    pub mod environment;
    pub mod order;
    pub mod create_predicate;
}

use utils::{build_take_order,environment,order,create_predicate};
use fuels::prelude::*;
use rand::Fill;
use std::mem::size_of;
use crate::utils::build_take_order::LimitOrder;
use fuel_core_interfaces::common::fuel_crypto::SecretKey;

use fuels::{
    prelude::{AssetConfig}, 
    tx::{Address, AssetId, Input, Output, Receipt, Transaction, TxPointer, UtxoId, Word, ContractId},
    test_helpers::{setup_custom_assets_coins, setup_test_provider, Config},
};


pub async fn setup_environment(
    coin: (Word, AssetId),
) -> (WalletUnlocked, WalletUnlocked, Vec<Input>, Provider) {
    const SIZE_SECRET_KEY: usize = size_of::<SecretKey>();
    const PADDING_BYTES: usize = SIZE_SECRET_KEY - size_of::<u64>();
    let mut secret_key: [u8; SIZE_SECRET_KEY] = [0; SIZE_SECRET_KEY];
    secret_key[PADDING_BYTES..].copy_from_slice(&(8320147306839812359u64).to_be_bytes());
    let mut wallet = WalletUnlocked::new_from_private_key(
        SecretKey::try_from(secret_key.as_slice())
            .expect("This should never happen as we provide a [u8; SIZE_SECRET_KEY] array"),
        None,
    );
    let mut wallet2 = WalletUnlocked::new_random(None);
    let mut all_coins: Vec<(UtxoId, Coin)> =
        setup_single_asset_coins(wallet.address(), coin.1, 1, coin.0);
    let mut coins2 = setup_single_asset_coins(wallet2.address(), coin.1, 1, coin.0 / 2);
    all_coins.append(&mut coins2);

    // Create the client and provider
    let mut provider_config = Config::local_node();
    provider_config.predicates = true;
    provider_config.utxo_validation = true;
    let (client, _) =
        setup_test_client(all_coins.clone(), Vec::new(), Some(provider_config), None).await;
    let provider = Provider::new(client);

    // Add provider to wallet
    wallet.set_provider(provider.clone());
    wallet2.set_provider(provider.clone());

    let coin_inputs: Vec<Input> = all_coins
        .into_iter()
        .map(|coin| Input::CoinSigned {
            utxo_id: UtxoId::from(coin.0.clone()),
            owner: Address::from(coin.1.owner.clone()),
            amount: coin.1.amount.clone().into(),
            asset_id: AssetId::from(coin.1.asset_id.clone()),
            tx_pointer: TxPointer::default(),
            witness_index: 0,
            maturity: 0,
        })
        .collect();
    (wallet, wallet2, coin_inputs, provider)
}
use fuel_core_interfaces::model::Coin;

    #[tokio::test]
    async fn test_limit_order_predicate() {
        let coin = (DEFAULT_COIN_AMOUNT, AssetId::default());
        let (maker, taker, coin_inputs, provider) = setup_environment(coin).await;
        let order = LimitOrder {
            maker: maker.address().into(),
            maker_amount: coin.0,
            taker_amount: coin.0 / 2,
            maker_token: Bits256::from_token(Token::B256([0u8; 32])).unwrap(),
            taker_token: Bits256::from_token(Token::B256([0u8; 32])).unwrap(),
            salt: 42,
        };
        let (predicate, predicate_input_coin) = order::create_order(&maker, &order, &provider).await;
        order::verify_balance_of_maker_and_predicate(
            &maker,
            predicate.address(),
            coin.1,
            coin.0,
            &provider,
        )
        .await;
        order::take_order(
            &taker,
            &order,
            &provider,
            predicate_input_coin,
            coin_inputs[0].clone(),
        )
        .await;
        order::verify_balance_post_swap(&maker, &taker, predicate.address(), order, &provider).await;
    }
    

