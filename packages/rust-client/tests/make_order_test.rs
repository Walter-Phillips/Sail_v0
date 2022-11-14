mod common;


use fuels::{
    prelude::*,
    tx::AssetId
};

use client::utils::{
    build_take_order::LimitOrder,
    order::create_order
};
    pub async fn verify_balance_of_maker_and_predicate(
        maker: &WalletUnlocked,
        predicate: &Bech32Address,
        asset: AssetId,
        amount: u64,
        provider: &Provider,
    ) {
        let balance = maker.get_asset_balance(&asset).await.unwrap();
        let predicate_balance = provider.get_asset_balance(predicate, asset).await.unwrap();
        assert!(balance == 0);
        assert!(predicate_balance == amount);
    }

    #[tokio::test]
    async fn test_make_order() {
        let (provider,maker, _, _, _, _, _) = common::setup().await;

        let order = LimitOrder {
            maker: maker.address().into(),
            maker_amount: 10,
            taker_amount: 10,
            maker_token: Bits256::from_token(Token::B256([0u8; 32])).unwrap(),
            taker_token: Bits256::from_token(Token::B256([0u8; 32])).unwrap(),
            salt: 22,
        };
        
        let (predicate, predicate_input_coins) = create_order(&maker, &order, &provider).await;
        verify_balance_of_maker_and_predicate(
            &maker,
            predicate.address(),
            AssetId::from_token(Token::B256([0u8; 32])).unwrap(),
            10,
            &provider,
        ).await;

        
    }
    
    

