//create the make order and the take order

use crate::utils::{build_cancel_order::*, build_take_order::*, create_predicate::*};

use fuels::contract::script::Script;
use fuels::{
    contract::predicate::Predicate,
    prelude::{Provider, TxParameters},
    signers::{Signer, WalletUnlocked},
    tx::{Address, AssetId, Input, TxPointer, UtxoId},
};

/// Gets the message to contract predicate

pub async fn create_order(
    maker: &WalletUnlocked,
    order: &LimitOrder,
    provider: &Provider,
) -> (Predicate, Input) {
    let (predicate, predicate_bytecode, predicate_root) = create_predicate(
        "0x7895d0059c0d0c1de8de15795191a1c1d01cd970db75fa42e15dc96e051b5570".to_string(),
        "1_000_000".to_string(),
        "0u8".to_string(),
        maker.address(),
        order.maker_amount,
        order.taker_amount,
        order.maker_token,
        order.taker_token,
        "123123".to_string(),
    );
    // create the order (fund the predicate)
    let (_tx, _rec) = maker
        .transfer(
            predicate.address(),
            order.maker_amount,
            AssetId::from(order.maker_token.0),
            TxParameters::default(),
        )
        .await
        .unwrap();
    let predicate_coin = &provider
        .get_coins(&predicate_root.into(), AssetId::default())
        .await
        .unwrap()[0];
    let predicate_coin_input = Input::CoinPredicate {
        utxo_id: UtxoId::from(predicate_coin.utxo_id.clone()),
        owner: predicate_root,
        amount: order.maker_amount,
        asset_id: AssetId::from(order.maker_token.0),
        tx_pointer: TxPointer::default(),
        maturity: 0,
        predicate: predicate_bytecode,
        predicate_data: vec![],
    };
    (predicate, predicate_coin_input)
}

pub async fn take_order(
    taker: &WalletUnlocked,
    order: &LimitOrder,
    provider: &Provider,
    gas_coin_inputs: Input,
    predicate_coin_input: Input,
) {
    let input_coins = &provider
        .get_coins(taker.address(), AssetId::default())
        .await
        .unwrap()[0];
    let taker_coin_input = Input::CoinSigned {
        utxo_id: UtxoId::from(input_coins.utxo_id.clone()),
        owner: taker.address().into(),
        amount: input_coins.amount.clone().into(),
        asset_id: input_coins.asset_id.clone().into(),
        tx_pointer: TxPointer::default(),
        witness_index: 0,
        maturity: 0,
    };
    println!(
        "amount: {}, utxoId: {:?}",
        predicate_coin_input.amount().unwrap(),
        predicate_coin_input.utxo_id()
    );
    let mut tx = build_take_order_tx(
        order,
        Address::from(taker.address()),
        gas_coin_inputs,
        predicate_coin_input,
        &vec![taker_coin_input],
        &[],
        TxParameters::default(),
    )
    .await;

    // Sign and execute the transaction
    taker.sign_transaction(&mut tx).await.unwrap();
    let script = Script::new(tx.clone());
    println!("This is what a transaction looks like, {:?}", tx);
    let _receipts = script.call(provider).await.unwrap();
}

pub async fn cancel_order(
    order: &LimitOrder,
    maker: &WalletUnlocked,
    provider: &Provider,
    gas_coin_inputs: Input,
    predicate_coin_input: Input,
    predicate_bytecode: Vec<u8>,
    predicate_root: Address,
) {
    let predicate_coin = &provider
        .get_coins(&predicate_root.into(), AssetId::default())
        .await
        .unwrap()[0];

    let maker_coin_input: Input = Input::CoinPredicate {
        utxo_id: UtxoId::from(predicate_coin.utxo_id.clone()),
        owner: maker.address().into(),
        amount: predicate_coin.amount.clone().into(),
        asset_id: predicate_coin.asset_id.clone().into(),
        tx_pointer: TxPointer::default(),
        maturity: 0,
        predicate: predicate_bytecode,
        predicate_data: vec![],
    };

    let mut tx = build_cancel_order_tx(
        order,
        Address::from(maker.address()),
        gas_coin_inputs,
        predicate_coin_input,
        &vec![maker_coin_input],
        &[],
        TxParameters::default(),
    )
    .await;

    // Sign and execute the transaction
    maker.sign_transaction(&mut tx).await.unwrap();
    let script = Script::new(tx);
    let _receipts = script.call(provider).await.unwrap();
}
