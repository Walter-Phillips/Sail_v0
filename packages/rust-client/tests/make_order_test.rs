mod common;


use fuels::{
    prelude::*,
    tx::{AssetId, Contract, Input, TxPointer, UtxoId, Address},
};

use client::utils::{
    order::*,
    build_take_order::*
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
        println!("This is the balance {}", balance);
        assert!(balance == 9999990);
        assert!(predicate_balance == amount);
    }

    // #[tokio::test]
    // async fn test_make_order() {
    //     let (provider,maker, _, _, _, _, _) = common::setup().await;

    //     let order = LimitOrder {
    //         maker: maker.address().into(),
    //         maker_amount: 10,
    //         taker_amount: 10,
    //         maker_token: Bits256::from_token(Token::B256([0u8; 32])).unwrap(),
    //         taker_token: Bits256::from_token(Token::B256([0u8; 32])).unwrap(),
    //         salt: 22,
    //     };
        
    //     let (predicate, _predicate_input_coins) = create_order(&maker, &order, &provider).await;
    //     verify_balance_of_maker_and_predicate(
    //         &maker,
    //         predicate.address(),
    //         AssetId::default(),
    //         10,
    //         &provider,
    //     ).await;
    // }

    #[tokio::test]
   async fn test_cancel_order() {
    
        let (provider,maker, _, _, _, _, _) = common::setup().await;

        let order = LimitOrder {
            maker: maker.address().into(),
            maker_amount: 10,
            taker_amount: 10,
            maker_token: Bits256::from_token(Token::B256([0u8; 32])).unwrap(),
            taker_token: Bits256::from_token(Token::B256([0u8; 32])).unwrap(),
            salt: 22,
        };

        let (predicate, predicate_coin_input) = create_order(&maker, &order, &provider).await;

        let gas_coin = &provider
        .get_spendable_coins(maker.address(), AssetId::default() , 1)
        .await
        .unwrap()[0];

        let gas_coin_utxo_id = gas_coin.utxo_id.clone().into();
        let gas_coin_amount: u64 = gas_coin.amount.clone().into();

        let gas_coin_inputs = Input::CoinSigned {
            utxo_id: gas_coin_utxo_id,
            tx_pointer: TxPointer::default(),
            owner: Address::from(maker.address()),
            amount: gas_coin_amount,
            asset_id: AssetId::default(),
            witness_index: 0,
            maturity: 0,
        };

        let swap_coin = &provider
        .get_spendable_coins(maker.address(), , 1)
        .await
        .unwrap()[0];
        let swap_coin_utxo_id = swap_coin.utxo_id.clone().into();
        let swap_coin_amount: u64 = swap_coin.amount.clone().into();


        let predicate_root: [u8; 32] = (*Contract::root_from_code(&predicate.code())).into();
        let predicate_root = Address::from(predicate_root); 

        cancel_order(&order, &maker, &provider, gas_coin_inputs, predicate_coin_input, predicate.code(), predicate_root).await;

        verify_balance_of_maker_and_predicate(
            &maker,
            predicate.address(),
            AssetId::from_token(Token::B256([0u8; 32])).unwrap(),
            10,
            &provider,
        ).await;


        
        
   }

    

