mod common;

use client::utils::create_predicate;

use fuels::{
    prelude::*,
    tx::AssetId
};

    #[tokio::test]
    async fn test_predicate_creation() {
        let (provider,wallet, address1, _, address2, _, address3) = common::setup().await;
        let (predicate, _, _) = create_predicate::create_predicate("0x7895d0059c0d0c1de8de15795191a1c1d01cd970db75fa42e15dc96e051b5570".to_string(),"1_000_000".to_string(),"0u8".to_string(),address1,312332,123123,address2,address3,"123123".to_string());

        let predicate_address  = predicate.address();
        let amount_to_predicate = 1000;
        let asset_id = AssetId::default();

        let _wallet_transfer_output = wallet
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
    

