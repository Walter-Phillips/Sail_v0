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
use crate::utils::build_take_order::LimitOrder;

use fuels::{
    prelude::{AssetConfig}, 
    tx::{Address, AssetId, Input, Output, Receipt, Transaction, TxPointer, UtxoId, Word, ContractId},
    test_helpers::{setup_custom_assets_coins, setup_test_provider, Config},
};
use fuel_core_interfaces::model::Coin;

    #[tokio::test]
    async fn test_make_order_predicate() {
        let mut wallet0 = Wallet::new_random(None);
        let mut wallet1 = Wallet::new_random(None);
        let mut rng = rand::thread_rng();
    
        let asset_base = AssetConfig {
            id: BASE_ASSET_ID,
            num_coins: 1,
            coin_amount: 1_000_000_000,
        };
    
        let mut asset_id_1 = AssetId::zeroed();
        asset_id_1.try_fill(&mut rng);
        let asset_1 = AssetConfig {
            id: asset_id_1,
            num_coins: 1,
            coin_amount: 100,
        };
    
        let mut asset_id_2 = AssetId::zeroed();
        asset_id_2.try_fill(&mut rng);
        let asset_2 = AssetConfig {
            id: asset_id_2,
            num_coins: 1,
            coin_amount:1_000,
        };
        
        let assets = vec![asset_base, asset_1, asset_2];
    
        let mut all_coins: Vec<(UtxoId, Coin)> =
            setup_custom_assets_coins(wallet0.address(), &assets);
        let mut coins2 = setup_custom_assets_coins(wallet1.address(),&assets);
        all_coins.append(&mut coins2);
    
        let (provider, _socket_addr) = setup_test_provider(all_coins.clone(), ,None).await;
        wallet0.set_provider(provider);
        wallet1.set_provider(provider);
    
        let order0 = build_take_order::LimitOrder {
            maker: wallet0.address().into(),
            maker_amount: wallet0.assets,   // needs to be the amount of asset 1 
            taker_amount: wallet1.assets,   //needs to be the amount of asset 2 
            maker_token: assets[1].id,
            taker_token: assets[2].id,
            salt: 12,
        };
        
        let order1 = build_take_order::LimitOrder {
            maker: wallet1.address().into(),
            maker_amount: wallet1.assets,   // needs to be the amount of asset 1 
            taker_amount: wallet0.assets,   //needs to be the amount of asset 2 
            maker_token: assets[2].id,
            taker_token: assets[1].id,
            salt: 12,
        };

    
        let (predicate, predicate_input_coin) = order::create_order(wallet1.address().into(), &order1, provider).await;
        
        order::verify_balance_of_maker_and_predicate(
            wallet0.address().into(),
            predicate.address(),
            assets[2].id,
            assets[1].coin_amount,
            provider,
        )
        .await;
    
        order::take_order(
            wallet1.address().into(),
            &order1,
            provider,
            predicate_input_coin,
            wallet1.assets.asset_base,
        )
        .await;
        order::verify_balance_post_swap(wallet0.address().into(), wallet1.address().into(), predicate.address(), order1, provider).await;
    
    }
    

