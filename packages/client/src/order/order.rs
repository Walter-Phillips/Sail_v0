use crate::utils::builder::LimitOrder;
use crate::utils::environment as env;
use crate::utils::create_predicate:create_predicate;

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
    let predicate = Predicate::load_from(PREDICATE).unwrap();
    let (predicate_bytecode, predicate_root) = get_predicate();

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
    let _receipt = env::take_order(
        order,
        &taker,
        gas_coin_inputs,
        predicate_coin_input,
        &vec![taker_coin_input],
        &vec![],
    )
    .await;
}

