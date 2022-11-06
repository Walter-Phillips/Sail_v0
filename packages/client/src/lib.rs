//test
mod utils {
    pub mod build_take_order;
    pub mod environment;
    pub mod order;
    pub mod create_predicate;
}
// us

mod success {

    #[test]
    fn test_make_order_predicate() {
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
    
        let order = build_take_order::LimitOrder {
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
