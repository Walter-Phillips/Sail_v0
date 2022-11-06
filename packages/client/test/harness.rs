//test
use fuels::{prelude::*, tx::ContractId};
use rand::Fill;
// Load abi from json
abigen!(MyContract, "out/debug/order-abi.json");
use crate::{utiles::environment as env, order::order as ord};

async fn get_contract_instance() -> (MyContract, ContractId) {
    // Launch a local network and deploy the contract
    let mut wallet0 = LocalWallet::new_random(None);
    let mut wallet1 = LocalWallet::new_random(None);

    let asset_base = AssetConfig {
        id: BASE_ASSET_ID,
        num_coins: 1,
        coin_amount: 1_000_000_000,
    };

    let mut asset_id_1 = AssetId::zeroed();
    asset_id_1.try_fill(&mut rng)?;
    let asset_1 = AssetConfig {
        id: asset_id_1,
        num_coins: 1,
        coin_amount: 100,
    };

    let mut asset_id_2 = AssetId::zeroed();
    asset_id_2.try_fill(&mut rng)?;
    let asset_2 = AssetConfig {
        id: asset_id_2,
        num_coins: 1,
        coin_amount:1_000,
    };
    .await;
    let assets = vec![asset_base, asset_1, asset_2];

    let coins = setup_custom_assets_coins(wallet.address(), assets);
    let (provider, _socket_addr) = setup_test_provider(coins.clone(), None).await;
    wallet0.set_provider(provider);
    wallet1.set_provider(provider);

    let id = Contract::deploy(
        "./out/debug/order.bin",
        &wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(
            "./out/debug/order-storage_slots.json".to_string(),
        )),
    )
    .await
    .unwrap();

    let instance = MyContract::new(id.to_string(), wallet);

    (instance, id.into())
}

#[tokio::test]

async fn can_get_contract_id() {
    let (_instance, _id) = get_contract_instance().await;

    // Now you have an instance of your contract you can use to test each function
}
mod success {
    use crate::utils::build_take_order::LimitOrder;

    #[tokio::test]
    #[test]
    async fn test_make_order_predicate() {
        let mut wallet0 = LocalWallet::new_random(None);
        let mut wallet1 = LocalWallet::new_random(None);
    
        let asset_base = AssetConfig {
            id: BASE_ASSET_ID,
            num_coins: 1,
            coin_amount: 1_000_000_000,
        };
    
        let mut asset_id_1 = AssetId::zeroed();
        asset_id_1.try_fill(&mut rng)?;
        let asset_1 = AssetConfig {
            id: asset_id_1,
            num_coins: 1,
            coin_amount: 100,
        };
    
        let mut asset_id_2 = AssetId::zeroed();
        asset_id_2.try_fill(&mut rng)?;
        let asset_2 = AssetConfig {
            id: asset_id_2,
            num_coins: 1,
            coin_amount:1_000,
        };
        
        let assets = vec![asset_base, asset_1, asset_2];
    
        let mut all_coins: Vec<(UtxoId, Coin)> =
            setup_custom_assets_coins(wallet0.address(), assets);
        let mut coins2 = setup_custom_assets_coins(wallet1.address(), assets);
        all_coins.append(&mut coins2);
    
        let (provider, _socket_addr) = setup_test_provider(all_coins.clone(), None).await;
        wallet0.set_provider(provider);
        wallet1.set_provider(provider);
    
        let order = LimitOrder {
            maker: wallet0.address().into(),
            maker_amount: wallet0.assets,   // needs to be the amount of asset 1 
            taker_amount: wallet1.assets,   //needs to be the amount of asset 2 
            maker_token: assets.asset_1,
            taker_token: assets.asset_2,
            salt: 69,
        };
    
        let (predicate, predicate_input_coin) = ord::create_order(&maker, &order, &provider).await;
        
        ord::verify_balance_of_maker_and_predicate(
            &maker,
            predicate.address(),
            assets.asset_2,
            assets.asset_1,
            &provider,
        )
        .await;
    
        ord::take_order(
            &taker,
            &order,
            &provider,
            predicate_input_coin,
            wallet1.assets.asset_base,
        )
        .await;
        ord::verify_balance_post_swap(&maker, &taker, predicate.address(), order, &provider).await;
    
    }
    
}
