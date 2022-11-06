//create the make order and the take order

use crate::utils::build_take_order::*;
use crate::utils::create_predicate::*;

use fuels::{
    contract::predicate::Predicate,
    prelude::{Bech32Address, Provider, TxParameters},
    signers::WalletUnlocked,
    tx::{Address, AssetId, Contract, Input, TxPointer, UtxoId},
};

/// Gets the message to contract predicate

pub async fn create_order(
    maker: &WalletUnlocked,
    order: &LimitOrder,
    provider: &Provider,
) -> (Predicate, Input) {
    create_predicate("0x7895d0059c0d0c1de8de15795191a1c1d01cd970db75fa42e15dc96e051b5570", "1_000_000", "0u8", maker.address(), order.maker_amount, order.taker_amount, order.maker_token, order.taker_token,"salt.to_string()");
    let predicate_bytecode = compile_to_bytes("order-predicate.sw", true).into_bytes();
    let predicate = Predicate::new(predicate_bytecode.clone());
    let predicate_root = Address::from(*Contract::root_from_code(predicate_bytecode.clone()));
    
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
        predicate: predicate_bytecode.clone(),
        predicate_data: vec![],
    };
    (predicate, predicate_coin_input)
}

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
    println!("{}", balance);
    assert!(predicate_balance == amount);
    println!("{}",predicate_balance);
}

pub async fn take_order(
    taker: &WalletUnlocked,
    order: &LimitOrder,
    provider: &Provider,
    predicate_coin_input: Input,
    gas_coin_inputs: Input,
) {
    let input_coins = &provider
        .get_coins(&taker.address(), AssetId::default())
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
    let _receipt = inner_take_order(
        order,
        &taker,
        gas_coin_inputs,
        predicate_coin_input,
        &vec![taker_coin_input],
        &vec![],
    )
    .await;
}
pub async fn verify_balance_post_swap(
    maker: &WalletUnlocked,
    taker: &WalletUnlocked,
    predicate_address: &Bech32Address,
    order: LimitOrder,
    provider: &Provider,
) {
    let maker_token = AssetId::from(order.maker_token.0);
    let taker_token = AssetId::from(order.taker_token.0);
    let balance_maker = maker.get_asset_balance(&taker_token).await.unwrap();
    let balance_taker = taker.get_asset_balance(&maker_token).await.unwrap();
    let predicate_balance = provider
        .get_asset_balance(predicate_address, maker_token)
        .await
        .unwrap();
    assert!(balance_maker == order.taker_amount);
    assert!(balance_taker == order.maker_amount);
    assert!(predicate_balance == 0);
}
